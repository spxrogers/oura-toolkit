//! Data-plane plumbing (#9): the generated `oura-toolkit-api` client wired through
//! `oura-toolkit-auth`, with 401-refresh-retry and cursor pagination.
//!
//! No hand-rolled HTTP: every request goes through the generated client; this module only
//! supplies auth (Bearer header from `TokenManager`), the one 401 retry, the bounded 429
//! wait-and-retry with its per-invocation budget (#28), the `next_token` loop, and date
//! parsing. All of it is hermetically tested against wiremock (see
//! `commands.rs` tests) — including the literal query strings the generated client emits.

use std::future::Future;

use anyhow::{bail, Context, Result};
use oura_toolkit_auth::{AuthError, TokenManager};

use crate::contract::UsageError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

/// The Oura API base URL (scheme + host; the generated client appends `/v2/...` paths).
///
/// This is the data-plane server from the spec's (overlay-fixed) `servers[0]` and is also
/// recorded in CLAUDE.md → PROJECT. It is deliberately a constant here rather than read at
/// build time: the PRISTINE vendored spec carries the upstream `api.None.com` bug that
/// only the overlay corrects, so spec-reading would vendor the bug. Tests inject their own
/// base URL; the constant is only bound in `main`.
pub const API_BASE: &str = "https://api.ouraring.com";

// Environment overrides for headless/CI/container use (#20). The lookup is injected so the
// precedence logic is unit-tested without touching process env (CLAUDE.md TESTING rule 2).

/// The data-plane base URL, honoring `OURA_API_BASE_URL` (point at a proxy, an alternate
/// Oura host, or a mock) over the built-in [`API_BASE`]. A trailing slash is trimmed so the
/// generated client's `/v2/...` paths never double up. Empty/whitespace is ignored.
pub fn base_url_from_env(env: impl Fn(&str) -> Option<String>) -> String {
    match env("OURA_API_BASE_URL") {
        Some(v) if !v.trim().is_empty() => v.trim().trim_end_matches('/').to_string(),
        _ => API_BASE.to_string(),
    }
}

/// The auth source for data commands and the MCP server, honoring `OURA_ACCESS_TOKEN` (a
/// raw OAuth access token — Oura deprecated personal access tokens in 2025) over the stored
/// tokens. The env token bypasses the store and never refreshes: when it is rejected the
/// manager reports [`AuthError::StaticTokenRejected`] (see [`crate::contract::classify`]).
/// Precedence: a non-empty `OURA_ACCESS_TOKEN` wins over any stored login. Empty/whitespace
/// is ignored (falls back to the store). NOT honored by the `oura auth` account commands,
/// which operate on the store itself.
pub fn manager_from_env(env: impl Fn(&str) -> Option<String>) -> Result<TokenManager, AuthError> {
    match access_token_override(env) {
        Some(token) => Ok(TokenManager::from_access_token(token)),
        None => TokenManager::load(),
    }
}

/// The trimmed `OURA_ACCESS_TOKEN` override, or `None` to fall back to the store. Split from
/// [`manager_from_env`] so the precedence rule (a non-empty value wins; whitespace-only is
/// ignored) is unit-tested without constructing a manager or touching the real store.
pub fn access_token_override(env: impl Fn(&str) -> Option<String>) -> Option<String> {
    env("OURA_ACCESS_TOKEN")
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

/// Build a generated-API client whose every request carries a fresh Bearer token.
///
/// The token is resolved NOW (refreshing if expired, #22 semantics); the client is cheap
/// and rebuilt per attempt so a post-refresh retry picks up the rotated token.
pub async fn authorized_client(
    manager: &TokenManager,
    base_url: &str,
) -> Result<oura_toolkit_api::Client> {
    let token = manager.access_token().await?;
    let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
        .context("access token contains bytes not valid in an HTTP header")?;
    value.set_sensitive(true); // keep the token out of debug/log output
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(AUTHORIZATION, value);
    let http = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("building the HTTP client")?;
    Ok(oura_toolkit_api::Client::new_with_client(base_url, http))
}

/// Honor `Retry-After` only up to this many seconds (#28): a short throttle is waited out
/// ONCE; anything longer (or a second 429) surfaces as the typed [`RateLimited`] error
/// immediately — one bounded wait-and-retry, never a retry storm. Deliberately below the
/// 30s per-request timeout so a rate-limited command still finishes promptly.
pub const RATE_LIMIT_WAIT_CAP_SECS: u64 = 10;

/// How many rate-limit waits ONE command invocation may spend in total (#28 review):
/// pagination re-enters [`with_auth_retry`] once per page, so without a shared budget a
/// throttle-then-succeed server could stretch a single command to `MAX_PAGES ×`
/// [`RATE_LIMIT_WAIT_CAP_SECS`] (~2.8 h) of sleeps. With it, added wall time is capped at
/// `RATE_LIMIT_MAX_WAITS × RATE_LIMIT_WAIT_CAP_SECS` (30 s) per invocation.
pub const RATE_LIMIT_MAX_WAITS: u32 = 3;

/// The per-invocation wait allowance. Each data command / MCP tool call creates ONE and
/// shares it across every page fetch; when it runs dry, a 429 fails typed immediately
/// instead of sleeping.
#[derive(Debug)]
pub struct RateLimitBudget(std::sync::atomic::AtomicU32);

impl RateLimitBudget {
    pub fn new() -> Self {
        Self(std::sync::atomic::AtomicU32::new(RATE_LIMIT_MAX_WAITS))
    }

    /// Consume one wait if any remain.
    fn try_take(&self) -> bool {
        self.0
            .fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |n| n.checked_sub(1),
            )
            .is_ok()
    }
}

impl Default for RateLimitBudget {
    fn default() -> Self {
        Self::new()
    }
}

/// Typed rate-limit failure (#28): the Oura API answered 429 and the bounded retry (if
/// any) did not clear it. Carries the reset instant so every consumer — CLI stderr and
/// MCP tool errors alike — can say WHEN to come back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimited {
    /// When the quota window resets: `X-RateLimit-Reset` (epoch seconds) when the server
    /// sent it, else now + `Retry-After`, else unknown.
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for RateLimited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The contract's documented shape (docs/cli-contract.md → Error style): scripts
        // and the MCP model both read "rate limited until <time>" out of this line.
        match self.until {
            Some(until) => write!(
                f,
                "the Oura API rate limit was exceeded — rate limited until {}",
                until.format("%Y-%m-%dT%H:%M:%SZ")
            ),
            None => write!(
                f,
                "the Oura API rate limit was exceeded (HTTP 429; the server gave no reset time)"
            ),
        }
    }
}

impl std::error::Error for RateLimited {}

/// What a 429 response told us about retrying (parsed from the spec-documented headers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RateLimitInfo {
    /// Server-requested wait (`Retry-After`: integer seconds, or an HTTP-date).
    retry_after: Option<std::time::Duration>,
    /// Absolute reset instant (`X-RateLimit-Reset`: epoch seconds, preferred; else
    /// derived from `Retry-After`).
    until: Option<chrono::DateTime<chrono::Utc>>,
}

impl RateLimitInfo {
    /// Parse the spec's rate-limit response headers. Unparseable values read as absent —
    /// a hostile or garbled header must degrade to "don't wait", never to a huge sleep.
    fn from_headers(headers: &HeaderMap, now: chrono::DateTime<chrono::Utc>) -> Self {
        let header_str = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());
        let retry_after = header_str("retry-after").and_then(|v| {
            let v = v.trim();
            v.parse::<u64>()
                .ok()
                .map(std::time::Duration::from_secs)
                .or_else(|| {
                    // RFC 7231 also allows an HTTP-date form.
                    chrono::DateTime::parse_from_rfc2822(v)
                        .ok()
                        .and_then(|at| (at.to_utc() - now).to_std().ok())
                })
        });
        let until = header_str("x-ratelimit-reset")
            .and_then(|v| v.trim().parse::<i64>().ok())
            .and_then(|epoch| chrono::DateTime::from_timestamp(epoch, 0))
            .or_else(|| {
                retry_after
                    .and_then(|d| chrono::TimeDelta::from_std(d).ok().map(|delta| now + delta))
            });
        Self { retry_after, until }
    }

    /// The wait to perform, if the server's ask fits the bound; `None` = fail now.
    fn bounded_wait(&self) -> Option<std::time::Duration> {
        self.retry_after
            .filter(|d| *d <= std::time::Duration::from_secs(RATE_LIMIT_WAIT_CAP_SECS))
    }
}

impl From<RateLimitInfo> for RateLimited {
    fn from(info: RateLimitInfo) -> Self {
        Self { until: info.until }
    }
}

/// One attempt's failure, split so the 429 wrapper can see rate limiting distinctly.
enum AttemptError {
    RateLimited(RateLimitInfo),
    Other(anyhow::Error),
}

/// Run one generated-client call with the contract's full retry semantics:
///
/// - **401** → force a refresh (adopting another process's rotation if one exists, #22)
///   and retry exactly once; a second 401 means the stored tokens are genuinely dead →
///   typed [`AuthError::NotAuthenticated`] (CLI contract: exit 4 + login hint).
/// - **429** (#28) → honor `Retry-After` up to [`RATE_LIMIT_WAIT_CAP_SECS`] and retry
///   exactly once, spending one unit of the caller's shared [`RateLimitBudget`]; a second
///   429, a longer ask, a missing `Retry-After`, or an exhausted budget → typed
///   [`RateLimited`] naming the reset time. Never more than one rate-limit retry per
///   call, never more than [`RATE_LIMIT_MAX_WAITS`] waits per command invocation.
pub async fn with_auth_retry<T, E, F, Fut>(
    manager: &TokenManager,
    base_url: &str,
    budget: &RateLimitBudget,
    call: F,
) -> Result<T>
where
    F: Fn(oura_toolkit_api::Client) -> Fut,
    Fut: Future<Output = Result<T, oura_toolkit_api::Error<E>>>,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    match attempt(manager, base_url, &call).await {
        Ok(value) => Ok(value),
        Err(AttemptError::RateLimited(info)) => match info
            .bounded_wait()
            .filter(|_| budget.try_take())
        {
            Some(wait) => {
                tokio::time::sleep(wait).await;
                match attempt(manager, base_url, &call).await {
                    Ok(value) => Ok(value),
                    // Still throttled after the server's own suggested wait: report,
                    // preferring the SECOND response's reset info (it is fresher).
                    Err(AttemptError::RateLimited(info2)) => Err(RateLimited::from(info2).into()),
                    Err(AttemptError::Other(e)) => Err(e),
                }
            }
            None => Err(RateLimited::from(info).into()),
        },
        Err(AttemptError::Other(e)) => Err(e),
    }
}

/// The auth-retrying core: one call, with the single 401-refresh-retry.
async fn attempt<T, E, F, Fut>(
    manager: &TokenManager,
    base_url: &str,
    call: &F,
) -> Result<T, AttemptError>
where
    F: Fn(oura_toolkit_api::Client) -> Fut,
    Fut: Future<Output = Result<T, oura_toolkit_api::Error<E>>>,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let auth = |e: anyhow::Error| AttemptError::Other(e);
    let client = authorized_client(manager, base_url).await.map_err(auth)?;
    match call(client).await {
        Ok(value) => Ok(value),
        Err(e) if is_unauthorized(&e) => {
            manager
                .force_refresh()
                .await
                .map_err(|e| AttemptError::Other(e.into()))?;
            let client = authorized_client(manager, base_url).await.map_err(auth)?;
            match call(client).await {
                Ok(value) => Ok(value),
                Err(e2) if is_unauthorized(&e2) => {
                    Err(AttemptError::Other(AuthError::NotAuthenticated.into()))
                }
                Err(e2) => Err(classify_api_error(e2)),
            }
        }
        Err(e) => Err(classify_api_error(e)),
    }
}

fn is_unauthorized<E>(e: &oura_toolkit_api::Error<E>) -> bool {
    e.status() == Some(reqwest::StatusCode::UNAUTHORIZED)
}

/// Map a generated-client error: 429s become [`AttemptError::RateLimited`] (with the
/// spec's guidance headers parsed); everything else keeps the contract's single-line
/// anyhow shape.
fn classify_api_error<E: std::fmt::Debug + Send + Sync + 'static>(
    e: oura_toolkit_api::Error<E>,
) -> AttemptError {
    // 429 has no schema-defined body in the (overlay-pruned) spec, so it always arrives
    // as `UnexpectedResponse` — the raw `reqwest::Response`, headers included.
    if let oura_toolkit_api::Error::UnexpectedResponse(r) = &e {
        if r.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return AttemptError::RateLimited(RateLimitInfo::from_headers(
                r.headers(),
                chrono::Utc::now(),
            ));
        }
    }
    AttemptError::Other(match e.status() {
        Some(status) => anyhow::anyhow!("Oura API returned HTTP {status}"),
        None => anyhow::anyhow!("request to the Oura API failed: {e}"),
    })
}

/// Follow `next_token` cursor pagination to completion, aggregating every page's records.
///
/// Guards (both tested): a page whose `next_token` equals the token that FETCHED it would
/// loop forever → error; a runaway cursor chain is capped at [`MAX_PAGES`].
pub const MAX_PAGES: usize = 1000;

pub async fn paginate<T, F, Fut>(mut fetch: F) -> Result<Vec<T>>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, Option<String>)>>,
{
    let mut all = Vec::new();
    let mut token: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let request_token = token.clone();
        let (mut page, next) = fetch(token.take()).await?;
        all.append(&mut page);
        match next {
            None => return Ok(all),
            Some(next) => {
                if Some(&next) == request_token.as_ref() {
                    bail!("pagination did not advance (server repeated next_token {next:?})");
                }
                token = Some(next);
            }
        }
    }
    bail!("pagination exceeded {MAX_PAGES} pages — refusing to follow a runaway cursor");
}

/// Parse a CLI date: `today`, `yesterday`, or `YYYY-MM-DD` — in the USER'S LOCAL timezone
/// (Oura data is user-local; see docs/cli-contract.md → Dates). Failures are
/// [`UsageError`]s (exit 2): the invocation is wrong, not the runtime.
pub fn parse_date(input: &str, today: chrono::NaiveDate) -> Result<chrono::NaiveDate> {
    match input {
        "today" => Ok(today),
        "yesterday" => Ok(today - chrono::Days::new(1)),
        other => chrono::NaiveDate::parse_from_str(other, "%Y-%m-%d").map_err(|_| {
            UsageError(format!(
                "invalid date {other:?} — expected today, yesterday, or YYYY-MM-DD"
            ))
            .into()
        }),
    }
}

/// Today in the user's local timezone.
pub fn local_today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}

/// The resolved date range for a data command. Defaults to the last 7 days ending today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
}

impl DateRange {
    pub fn resolve(
        start: Option<&str>,
        end: Option<&str>,
        today: chrono::NaiveDate,
    ) -> Result<Self> {
        let end_date = match end {
            Some(e) => parse_date(e, today)?,
            None => today,
        };
        let start_date = match start {
            Some(s) => parse_date(s, today)?,
            None => end_date - chrono::Days::new(6),
        };
        if start_date > end_date {
            return Err(
                UsageError(format!("--start {start_date} is after --end {end_date}")).into(),
            );
        }
        Ok(Self {
            start: start_date,
            end: end_date,
        })
    }

    /// Start of the start day / end of the end day, as UTC instants (for the
    /// datetime-parameterized endpoints like heartrate), interpreted in local time.
    pub fn as_utc_bounds(&self) -> (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) {
        self.utc_bounds_in(&chrono::Local)
    }

    /// [`Self::as_utc_bounds`] generalized over the timezone, so the conversion —
    /// including its DST edge cases — is unit-testable without mutating process state.
    ///
    /// DST can make a wall-clock time nonexistent (spring-forward gap — some zones
    /// historically skip midnight itself, e.g. America/Sao_Paulo 2017-10-15) or ambiguous
    /// (fall-back repeat). Ambiguity resolves outward — earliest start, latest end — to
    /// keep the requested window maximal. A nonexistent time is nudged INTO the day in
    /// 15-minute steps until it exists, landing on the first (start) or last (end)
    /// instant that actually occurred on that local day. Never panics.
    pub(crate) fn utc_bounds_in<Tz: chrono::TimeZone>(
        &self,
        tz: &Tz,
    ) -> (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) {
        let start = self
            .start
            .and_hms_opt(0, 0, 0)
            .expect("midnight is a valid time");
        let end = self
            .end
            .and_hms_opt(23, 59, 59)
            .expect("23:59:59 is a valid time");
        (
            resolve_local(tz, start, true).with_timezone(&chrono::Utc),
            resolve_local(tz, end, false).with_timezone(&chrono::Utc),
        )
    }
}

/// Resolve a wall-clock time in `tz`, stepping `forward` (or backward) across a DST gap
/// until the time exists. Modern zone offsets change by 30/45/60/120 minutes — multiples
/// of the 15-minute step — so the scan lands on the transition instant itself; historical
/// seconds-level LMT gaps resolve to the next lattice point past the transition (≤15 min
/// inside the day — immaterial). Pathological-but-real edge, accepted: a date-line
/// day-skip (Pacific/Apia 2011-12-30 never existed) can nudge start past end, yielding an
/// inverted window and an empty (not wrong) result. The bound (100 steps ≈ 25h) is
/// unreachable for any real timezone; the final fallback merely keeps the function total
/// for a pathological `TimeZone` impl rather than panicking.
fn resolve_local<Tz: chrono::TimeZone>(
    tz: &Tz,
    wall: chrono::NaiveDateTime,
    forward: bool,
) -> chrono::DateTime<Tz> {
    let step = chrono::TimeDelta::minutes(15);
    let mut candidate = wall;
    for _ in 0..100 {
        let resolved = tz.from_local_datetime(&candidate);
        let picked = if forward {
            resolved.earliest()
        } else {
            resolved.latest()
        };
        if let Some(dt) = picked {
            return dt;
        }
        candidate = if forward {
            candidate + step
        } else {
            candidate - step
        };
    }
    tz.from_utc_datetime(&wall)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn day(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    // --- headless env overrides (#20) ---------------------------------------------------

    fn env_of<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |key| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn base_url_override_wins_and_trims_a_trailing_slash() {
        assert_eq!(
            base_url_from_env(env_of(&[("OURA_API_BASE_URL", "http://127.0.0.1:9/")])),
            "http://127.0.0.1:9",
            "trailing slash trimmed so /v2 paths don't double up"
        );
        // Empty / whitespace / absent all fall back to the built-in host.
        assert_eq!(
            base_url_from_env(env_of(&[("OURA_API_BASE_URL", "  ")])),
            API_BASE
        );
        assert_eq!(base_url_from_env(env_of(&[])), API_BASE);
    }

    #[test]
    fn access_token_override_takes_a_nonempty_value_and_ignores_blanks() {
        assert_eq!(
            access_token_override(env_of(&[("OURA_ACCESS_TOKEN", "  tok-123  ")])),
            Some("tok-123".to_string()),
            "surrounding whitespace (e.g. a stray newline from `export`) is trimmed"
        );
        assert_eq!(
            access_token_override(env_of(&[("OURA_ACCESS_TOKEN", "   ")])),
            None
        );
        assert_eq!(
            access_token_override(env_of(&[])),
            None,
            "absent ⇒ fall back to the store"
        );
    }

    #[tokio::test]
    async fn manager_from_env_uses_the_env_token_verbatim_without_a_store() {
        // The whole point of OURA_ACCESS_TOKEN: no store, no login — the injected token is
        // what every request carries.
        let manager = manager_from_env(env_of(&[("OURA_ACCESS_TOKEN", "env-tok")])).unwrap();
        assert_eq!(manager.access_token().await.unwrap(), "env-tok");
    }

    #[test]
    fn parses_today_yesterday_and_iso_dates() {
        let today = day("2026-07-02");
        assert_eq!(parse_date("today", today).unwrap(), day("2026-07-02"));
        assert_eq!(parse_date("yesterday", today).unwrap(), day("2026-07-01"));
        assert_eq!(parse_date("2026-06-15", today).unwrap(), day("2026-06-15"));
    }

    #[test]
    fn rejects_garbage_dates_with_an_actionable_message() {
        let err = parse_date("next tuesday", day("2026-07-02")).unwrap_err();
        assert!(
            err.to_string().contains("today, yesterday, or YYYY-MM-DD"),
            "{err}"
        );
    }

    #[test]
    fn range_defaults_to_last_seven_days_ending_today() {
        let r = DateRange::resolve(None, None, day("2026-07-02")).unwrap();
        assert_eq!(r.start, day("2026-06-26"));
        assert_eq!(r.end, day("2026-07-02"));
    }

    #[test]
    fn range_rejects_inverted_bounds() {
        let err =
            DateRange::resolve(Some("today"), Some("yesterday"), day("2026-07-02")).unwrap_err();
        assert!(err.to_string().contains("after"), "{err}");
    }

    #[tokio::test]
    async fn paginate_aggregates_in_order_and_stops_at_none() {
        let pages = std::sync::Mutex::new(vec![
            (vec![1, 2], Some("t1".to_string())),
            (vec![3], Some("t2".to_string())),
            (vec![4, 5], None),
        ]);
        let all = paginate(|_token| {
            let next = pages.lock().unwrap().remove(0);
            async move { Ok(next) }
        })
        .await
        .unwrap();
        assert_eq!(all, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn paginate_bails_on_a_non_advancing_token() {
        let err = paginate(|_token| async move { Ok((vec![1], Some("same".to_string()))) })
            .await
            .unwrap_err();
        assert!(err.to_string().contains("did not advance"), "{err}");
    }

    /// Fixed-offset zone: the documented local 00:00:00 / 23:59:59 window converts to
    /// exact UTC instants (docs/cli-contract.md → Dates).
    #[test]
    fn utc_bounds_convert_the_local_day_window_exactly() {
        let r = DateRange {
            start: day("2026-06-26"),
            end: day("2026-07-02"),
        };
        let tz = chrono::FixedOffset::east_opt(5 * 3600 + 1800).unwrap(); // +05:30
        let (start, end) = r.utc_bounds_in(&tz);
        assert_eq!(start.to_rfc3339(), "2026-06-25T18:30:00+00:00");
        assert_eq!(end.to_rfc3339(), "2026-07-02T18:29:59+00:00");
    }

    /// DST spring-forward gap AT midnight (America/Sao_Paulo 2017-10-15 skipped
    /// 00:00–00:59): must not panic, and must land on the first instant that actually
    /// existed that local day (01:00 -02:00).
    #[test]
    fn utc_bounds_survive_a_dst_gap_at_local_midnight() {
        let tz: chrono_tz::Tz = "America/Sao_Paulo".parse().unwrap();
        let r = DateRange {
            start: day("2017-10-15"),
            end: day("2017-10-15"),
        };
        let (start, end) = r.utc_bounds_in(&tz);
        assert_eq!(start.to_rfc3339(), "2017-10-15T03:00:00+00:00");
        assert_eq!(end.to_rfc3339(), "2017-10-16T01:59:59+00:00");
    }

    /// DST fall-back fold (Sao_Paulo 2018-02-18 00:00 fell back to 2018-02-17 23:00, so
    /// 23:00–23:59 happened twice): the end bound resolves to the LATER occurrence,
    /// keeping the requested window maximal.
    #[test]
    fn utc_bounds_resolve_a_dst_fold_outward() {
        let tz: chrono_tz::Tz = "America/Sao_Paulo".parse().unwrap();
        let r = DateRange {
            start: day("2018-02-17"),
            end: day("2018-02-17"),
        };
        let (_, end) = r.utc_bounds_in(&tz);
        assert_eq!(end.to_rfc3339(), "2018-02-18T02:59:59+00:00"); // 23:59:59 -03:00
    }

    /// The OTHER half of "folds resolve outward": a fold covering local MIDNIGHT
    /// (America/Havana fell back 2023-11-05 01:00→00:00, so 00:00–00:59 happened twice)
    /// must resolve the START bound to the EARLIER occurrence — an implementation picking
    /// `.latest()` for the start would clip the doubled hour and fail here.
    #[test]
    fn utc_bounds_resolve_a_start_fold_to_the_earlier_instant() {
        let tz: chrono_tz::Tz = "America/Havana".parse().unwrap();
        let r = DateRange {
            start: day("2023-11-05"),
            end: day("2023-11-05"),
        };
        let (start, _) = r.utc_bounds_in(&tz);
        assert_eq!(start.to_rfc3339(), "2023-11-05T04:00:00+00:00"); // 00:00:00 -04:00 (CDT)
    }

    // --- rate limiting (#28) -----------------------------------------------------------

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.insert(
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            );
        }
        h
    }

    fn now() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339("2026-07-04T18:00:00Z")
            .unwrap()
            .to_utc()
    }

    #[test]
    fn retry_after_integer_seconds_parses_and_derives_until() {
        let info = RateLimitInfo::from_headers(&headers(&[("retry-after", "42")]), now());
        assert_eq!(info.retry_after, Some(std::time::Duration::from_secs(42)));
        assert_eq!(
            info.until.unwrap().to_rfc3339(),
            "2026-07-04T18:00:42+00:00",
            "until derives from now + Retry-After when no reset header exists"
        );
    }

    #[test]
    fn retry_after_http_date_parses() {
        let info = RateLimitInfo::from_headers(
            &headers(&[("retry-after", "Sat, 04 Jul 2026 18:00:30 GMT")]),
            now(),
        );
        assert_eq!(info.retry_after, Some(std::time::Duration::from_secs(30)));
    }

    #[test]
    fn reset_epoch_wins_over_derived_until() {
        let info = RateLimitInfo::from_headers(
            // 2026-07-04T19:00:00Z = 1783191600
            &headers(&[("retry-after", "5"), ("x-ratelimit-reset", "1783191600")]),
            now(),
        );
        assert_eq!(
            info.until.unwrap().to_rfc3339(),
            "2026-07-04T19:00:00+00:00",
            "X-RateLimit-Reset is authoritative for the reset instant"
        );
        assert_eq!(info.retry_after, Some(std::time::Duration::from_secs(5)));
    }

    #[test]
    fn hostile_or_missing_headers_degrade_to_absent_never_to_a_wait() {
        // (Control bytes can't appear here: reqwest rejects them at the HeaderValue
        // layer, so "garbled" means printable-but-unparseable.)
        for bad in [
            &[("retry-after", "soon")][..],
            &[("retry-after", "-5")],
            &[("retry-after", "12.5")],
            &[("retry-after", "999999999999999999999999")],
            &[],
        ] {
            let info = RateLimitInfo::from_headers(&headers(bad), now());
            assert_eq!(info.retry_after, None, "{bad:?} must not produce a wait");
            assert_eq!(info.until, None);
            assert_eq!(
                info.bounded_wait(),
                None,
                "no parseable guidance ⇒ fail fast, never sleep"
            );
        }
    }

    #[test]
    fn a_parseable_but_huge_retry_after_cannot_panic_the_until_derivation() {
        // u64-valid but overflows chrono::TimeDelta: the `.ok()` guard on from_std must
        // degrade `until` to None (an `.unwrap()` regression panics right here), and the
        // cap keeps it unwaitable.
        let info =
            RateLimitInfo::from_headers(&headers(&[("retry-after", "99999999999999999")]), now());
        assert_eq!(
            info.retry_after,
            Some(std::time::Duration::from_secs(99_999_999_999_999_999))
        );
        assert_eq!(
            info.until, None,
            "overflowing delta must not fabricate an until"
        );
        assert_eq!(info.bounded_wait(), None);
    }

    #[test]
    fn budget_allows_exactly_max_waits_then_runs_dry() {
        let budget = RateLimitBudget::new();
        for i in 0..RATE_LIMIT_MAX_WAITS {
            assert!(budget.try_take(), "wait {i} must fit the budget");
        }
        assert!(
            !budget.try_take(),
            "the budget must run dry after MAX_WAITS"
        );
        assert!(!budget.try_take(), "and STAY dry (no underflow wraparound)");
    }

    #[test]
    fn bounded_wait_honors_the_cap() {
        let at =
            |secs: &str| RateLimitInfo::from_headers(&headers(&[("retry-after", secs)]), now());
        assert_eq!(
            at("10").bounded_wait(),
            Some(std::time::Duration::from_secs(RATE_LIMIT_WAIT_CAP_SECS)),
            "exactly the cap is still waitable"
        );
        assert_eq!(
            at("11").bounded_wait(),
            None,
            "one second past the cap ⇒ immediate typed error"
        );
        assert_eq!(at("0").bounded_wait(), Some(std::time::Duration::ZERO));
    }

    #[test]
    fn rate_limited_display_names_the_reset_time() {
        let with_until = RateLimited { until: Some(now()) };
        assert_eq!(
            with_until.to_string(),
            "the Oura API rate limit was exceeded — rate limited until 2026-07-04T18:00:00Z",
            "the documented 'rate limited until <time>' shape (docs/cli-contract.md)"
        );
        let without = RateLimited { until: None };
        assert!(without.to_string().contains("no reset time"), "{without}");
    }

    /// A server that mints a NEW token every page defeats the non-advancing check; the
    /// page cap must stop it after exactly MAX_PAGES fetches.
    #[tokio::test]
    async fn paginate_caps_a_runaway_cursor_at_max_pages() {
        let calls = std::sync::atomic::AtomicUsize::new(0);
        let err = paginate(|_token| {
            let n = calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move { Ok((vec![n], Some(format!("t{n}")))) }
        })
        .await
        .unwrap_err();
        assert!(err.to_string().contains("exceeded"), "{err}");
        assert_eq!(
            calls.load(std::sync::atomic::Ordering::SeqCst),
            MAX_PAGES,
            "must stop fetching at the cap, not after it"
        );
    }
}
