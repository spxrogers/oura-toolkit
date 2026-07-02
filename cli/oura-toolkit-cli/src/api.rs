//! Data-plane plumbing (#9): the generated `oura-toolkit-api` client wired through
//! `oura-toolkit-auth`, with 401-refresh-retry and cursor pagination.
//!
//! No hand-rolled HTTP: every request goes through the generated client; this module only
//! supplies auth (Bearer header from `TokenManager`), the one 401 retry, the `next_token`
//! loop, and date parsing. All of it is hermetically tested against wiremock (see
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

/// Run one generated-client call with the contract's auth semantics: on a 401, force a
/// refresh (adopting another process's rotation if one exists, #22) and retry exactly
/// once. A second 401 means the stored tokens are genuinely dead → typed
/// [`AuthError::NotAuthenticated`], which the CLI contract maps to exit 4 + login hint.
pub async fn with_auth_retry<T, E, F, Fut>(
    manager: &TokenManager,
    base_url: &str,
    call: F,
) -> Result<T>
where
    F: Fn(oura_toolkit_api::Client) -> Fut,
    Fut: Future<Output = Result<T, oura_toolkit_api::Error<E>>>,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let client = authorized_client(manager, base_url).await?;
    match call(client).await {
        Ok(value) => Ok(value),
        Err(e) if is_unauthorized(&e) => {
            manager.force_refresh().await?;
            let client = authorized_client(manager, base_url).await?;
            match call(client).await {
                Ok(value) => Ok(value),
                Err(e2) if is_unauthorized(&e2) => Err(AuthError::NotAuthenticated.into()),
                Err(e2) => Err(api_error(e2)),
            }
        }
        Err(e) => Err(api_error(e)),
    }
}

fn is_unauthorized<E>(e: &oura_toolkit_api::Error<E>) -> bool {
    e.status() == Some(reqwest::StatusCode::UNAUTHORIZED)
}

/// Map a generated-client error to a single-line anyhow error per the contract's style.
fn api_error<E: std::fmt::Debug + Send + Sync + 'static>(
    e: oura_toolkit_api::Error<E>,
) -> anyhow::Error {
    match e.status() {
        Some(status) => anyhow::anyhow!("Oura API returned HTTP {status}"),
        None => anyhow::anyhow!("request to the Oura API failed: {e}"),
    }
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
        use chrono::TimeZone as _;
        let start_local = chrono::Local
            .from_local_datetime(&self.start.and_hms_opt(0, 0, 0).expect("valid midnight"))
            .earliest()
            .expect("local midnight exists");
        let end_local = chrono::Local
            .from_local_datetime(&self.end.and_hms_opt(23, 59, 59).expect("valid time"))
            .latest()
            .expect("local end-of-day exists");
        (
            start_local.with_timezone(&chrono::Utc),
            end_local.with_timezone(&chrono::Utc),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn day(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
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
