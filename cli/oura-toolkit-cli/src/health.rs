//! The local health-store commands (`oura sync` / `oura import …` / `oura capacity` /
//! `oura context`) — the CLI face of the `oura-toolkit-health` crate.
//!
//! Same split as `commands.rs`: `fetch_*`-shaped functions return the crate's data
//! models for BOTH consumers (rendered CLI commands here, MCP tools in `mcp.rs`), and
//! rendering goes through the output layer's single entry points. The store path comes
//! from the environment (`XDG_DATA_HOME`/`HOME`/`LOCALAPPDATA`), so binary-level tests
//! isolate it exactly like the token store.
//!
//! Safety behaviors owned here (vision doc §1's threat model, enforced by tests):
//! - Opening the store for a WRITE (sync/import) prints the cloud-sync-root warning to
//!   stderr when the data dir would replicate off the machine.
//! - `oura import apple-health` offers `--remove-source` and, when not used, reminds
//!   the user (stderr) that the plaintext export still exists.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context as _, Result};
use chrono::NaiveDate;

use oura_toolkit_auth::TokenManager;
use oura_toolkit_health::engine::{self, AnalogReport, CapacityReport, WeekLoad};
use oura_toolkit_health::{apple, calendar, habits, toggl, DayMap, HealthStore, OuraDay};

use crate::api::{parse_date, DateRange};
use crate::commands::{fetch_activity, fetch_readiness, fetch_sleep, fetch_stress};
use crate::contract::UsageError;
use crate::output::{render_record, render_result, RenderOptions, Table};

/// `oura sync` covers this many days when `--start` is omitted: enough history for the
/// engine's baselines without hammering the API on a first run.
pub const SYNC_DEFAULT_DAYS: u64 = 90;

/// Open the store at the fixed data path, surfacing the cloud-sync warning on stderr
/// when `warn_sync_root` (write paths only — the MCP server stays quiet on stderr).
pub fn open_store(warn_sync_root: bool) -> Result<HealthStore> {
    let store = HealthStore::open_default().context("locating the health data store")?;
    if warn_sync_root {
        if let Some(warning) = oura_toolkit_health::store::sync_root_warning(store.dir()) {
            eprintln!("{warning}");
        }
    }
    Ok(store)
}

/// Resolve a range like [`DateRange::resolve`] but with a command-specific default
/// span (sync wants [`SYNC_DEFAULT_DAYS`], not the data commands' 7).
pub fn resolve_range_with_default(
    start: Option<&str>,
    end: Option<&str>,
    today: NaiveDate,
    default_days: u64,
) -> Result<DateRange> {
    let end_date = match end {
        Some(s) => parse_date(s, today)?,
        None => today,
    };
    let start_date = match start {
        Some(s) => parse_date(s, today)?,
        None => end_date - chrono::Days::new(default_days - 1),
    };
    if start_date > end_date {
        return Err(UsageError(format!("--start {start_date} is after --end {end_date}")).into());
    }
    Ok(DateRange {
        start: start_date,
        end: end_date,
    })
}

/// What an import/sync did, for the summary line and `--json`.
#[derive(serde::Serialize)]
pub struct ImportSummary {
    pub source: &'static str,
    /// Days present in the input (added + updated + unchanged).
    pub days: u32,
    pub added: u32,
    pub updated: u32,
    pub unchanged: u32,
    pub store_path: String,
}

fn summarize(
    source: &'static str,
    stats: oura_toolkit_health::UpsertStats,
    store: &HealthStore,
    render: RenderOptions,
) -> Result<String> {
    let summary = ImportSummary {
        source,
        days: stats.added + stats.updated + stats.unchanged,
        added: stats.added,
        updated: stats.updated,
        unchanged: stats.unchanged,
        store_path: store.dir().display().to_string(),
    };
    let fields = [
        ("source", summary.source.to_string()),
        (
            "days",
            format!(
                "{} ({} added, {} updated, {} unchanged)",
                summary.days, summary.added, summary.updated, summary.unchanged
            ),
        ),
        ("store", summary.store_path.clone()),
    ];
    render_record(&summary, &fields, render)
}

/// Shared data plane for `oura sync`: pull the four Oura dailies and fold them into
/// per-day [`OuraDay`] records (returned, not yet stored — the caller upserts).
pub async fn fetch_oura_days(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<BTreeMap<NaiveDate, OuraDay>> {
    // The four fetches are independent; run them concurrently like a client would.
    let (sleep, readiness, activity, stress) = tokio::try_join!(
        fetch_sleep(manager, base_url, range),
        fetch_readiness(manager, base_url, range),
        fetch_activity(manager, base_url, range),
        fetch_stress(manager, base_url, range),
    )?;

    let mut days: BTreeMap<NaiveDate, OuraDay> = BTreeMap::new();
    let day_of = |iso: &str| -> Result<NaiveDate> {
        iso.parse::<NaiveDate>()
            .with_context(|| format!("Oura returned an unparseable day {iso:?}"))
    };
    for doc in &sleep {
        let entry = days.entry(day_of(&doc.day)?).or_default();
        entry.sleep_score = doc.score.map(|s| s as f64);
    }
    for doc in &readiness {
        let entry = days.entry(day_of(&doc.day)?).or_default();
        entry.readiness_score = doc.score.map(|s| s as f64);
        entry.temperature_deviation_c = doc.temperature_deviation;
    }
    for doc in &activity {
        let entry = days.entry(day_of(&doc.day)?).or_default();
        entry.activity_score = doc.score.map(|s| s as f64);
        entry.steps = Some(doc.steps as f64);
    }
    for doc in &stress {
        let entry = days.entry(day_of(&doc.day)?).or_default();
        entry.stress_high_seconds = doc.stress_high.map(|s| s as f64);
        entry.recovery_high_seconds = doc.recovery_high.map(|s| s as f64);
    }
    Ok(days)
}

/// `oura sync`: Oura dailies → the local store.
pub async fn sync(
    manager: &TokenManager,
    base_url: &str,
    store: &HealthStore,
    range: DateRange,
    render: RenderOptions,
) -> Result<String> {
    let days = fetch_oura_days(manager, base_url, range).await?;
    let stats = store.upsert(days).context("writing the health store")?;
    summarize("Oura", stats, store, render)
}

/// `oura import apple-health`.
pub fn import_apple(
    store: &HealthStore,
    file: &Path,
    remove_source: bool,
    render: RenderOptions,
) -> Result<String> {
    let days = apple::import_apple_export(file)
        .with_context(|| format!("importing {}", file.display()))?;
    let stats = store.upsert(days).context("writing the health store")?;
    if remove_source {
        std::fs::remove_file(file)
            .with_context(|| format!("removing the imported export {}", file.display()))?;
        eprintln!("removed {}", file.display());
    } else {
        // Threat-model reminder (vision doc §1): the plaintext export outside the store
        // is the highest-exposure copy.
        eprintln!(
            "note: {} still exists — it holds your full plaintext health history; \
             delete it once you no longer need it (or re-run with --remove-source)",
            file.display()
        );
    }
    summarize("Apple Health", stats, store, render)
}

/// `oura import calendar`.
pub fn import_calendar(
    store: &HealthStore,
    file: &Path,
    today: NaiveDate,
    render: RenderOptions,
) -> Result<String> {
    let days = calendar::import_ics(file, today)
        .with_context(|| format!("importing {}", file.display()))?;
    let stats = store.upsert(days).context("writing the health store")?;
    summarize("calendar", stats, store, render)
}

/// `oura import toggl`.
pub fn import_toggl(store: &HealthStore, file: &Path, render: RenderOptions) -> Result<String> {
    let days =
        toggl::import_toggl_csv(file).with_context(|| format!("importing {}", file.display()))?;
    let stats = store.upsert(days).context("writing the health store")?;
    summarize("Toggl", stats, store, render)
}

// ---------------------------------------------------------------------------------------
// Engine fetchers — the shared data plane for `oura capacity`/`oura context` AND the MCP
// tools (same one-data-plane rule as commands.rs).
// ---------------------------------------------------------------------------------------

pub fn fetch_capacity(store: &HealthStore, today: NaiveDate) -> Result<CapacityReport> {
    let days = store.load().context("reading the health store")?;
    engine::capacity(&days, today).context("computing capacity")
}

pub fn fetch_analogs(store: &HealthStore, week: NaiveDate, k: usize) -> Result<AnalogReport> {
    let days = store.load().context("reading the health store")?;
    engine::find_analogs(&days, week, k).context("matching analog weeks")
}

pub fn fetch_upcoming(store: &HealthStore, today: NaiveDate, weeks: u32) -> Result<Vec<WeekLoad>> {
    let days = store.load().context("reading the health store")?;
    Ok(engine::upcoming_load(&days, today, weeks))
}

pub fn fetch_day_context(store: &HealthStore, range: DateRange) -> Result<DayMap> {
    let days = store.load().context("reading the health store")?;
    Ok(days
        .into_iter()
        .filter(|(day, _)| *day >= range.start && *day <= range.end)
        .collect())
}

/// Shared data plane for `oura habit stats` AND the `get_habits` MCP tool.
pub fn fetch_habits(store: &HealthStore, today: NaiveDate) -> Result<Vec<habits::HabitStats>> {
    let days = store.load().context("reading the health store")?;
    Ok(habits::habit_stats(&days, today))
}

/// A habit write's outcome, for the CLI line and the `log_habit` MCP result.
#[derive(serde::Serialize)]
pub struct HabitLogOutcome {
    /// The canonical (normalized) habit name that was written.
    pub habit: String,
    pub date: NaiveDate,
    /// True after a `log`, false after an `undo`.
    pub logged: bool,
    /// False when the operation was a no-op (already logged / nothing to undo).
    pub changed: bool,
}

/// Log or undo one habit for one day — the shared write path for `oura habit
/// log|undo` and the `log_habit` MCP tool. An invalid name is the CALLER's mistake:
/// reclassified as a usage error (exit 2 / `invalid_params`).
pub fn habit_write(
    store: &HealthStore,
    name: &str,
    date: NaiveDate,
    undo: bool,
) -> Result<HabitLogOutcome> {
    let result = if undo {
        store.unlog_habit(date, name)
    } else {
        store.log_habit(date, name)
    };
    let (habit, changed) = result.map_err(|e| match e {
        oura_toolkit_health::HealthError::InvalidHabitName { .. } => {
            anyhow::Error::new(UsageError(e.to_string()))
        }
        other => anyhow::Error::new(other).context("writing the health store"),
    })?;
    Ok(HabitLogOutcome {
        habit,
        date,
        logged: !undo,
        changed,
    })
}

/// `oura habit log` / `oura habit undo`: one confirmation line to stdout.
pub fn habit_write_cmd(
    store: &HealthStore,
    name: &str,
    date: NaiveDate,
    undo: bool,
    render: RenderOptions,
) -> Result<String> {
    let outcome = habit_write(store, name, date, undo)?;
    let status = match (outcome.logged, outcome.changed) {
        (true, true) => "logged".to_string(),
        (true, false) => "already logged".to_string(),
        (false, true) => "removed".to_string(),
        (false, false) => "nothing to remove".to_string(),
    };
    let fields = [
        ("habit", outcome.habit.clone()),
        ("date", outcome.date.to_string()),
        ("status", status),
    ];
    render_record(&outcome, &fields, render)
}

/// `oura habit stats`: the long-grain view — days/week over trailing windows.
pub fn habit_stats_cmd(
    store: &HealthStore,
    today: NaiveDate,
    render: RenderOptions,
) -> Result<String> {
    let stats = fetch_habits(store, today)?;
    let mut table = Table::new(["HABIT", "7D", "28D", "91D", "DAYS", "LAST"]);
    for s in &stats {
        table.row([
            s.name.clone(),
            format!("{:.1}", s.rate_7d),
            format!("{:.1}", s.rate_28d),
            format!("{:.1}", s.rate_91d),
            s.total_days.to_string(),
            s.last_logged.to_string(),
        ]);
    }
    render_result(&stats, &table, render)
}

/// `oura dashboard`: render the self-contained HTML from the store and (unless told
/// not to) open it in the default browser. The rendered path is the RESULT (stdout);
/// browser-opening failures are nonfatal prose (stderr) — the file exists either way.
pub fn dashboard(
    store: &HealthStore,
    today: NaiveDate,
    out: Option<&Path>,
    open_browser: bool,
) -> Result<String> {
    let days = store.load().context("reading the health store")?;
    let html = crate::dashboard::render(&days, today, &store.dir().display().to_string());
    let path = match out {
        Some(p) => p.to_path_buf(),
        None => store.dir().join("dashboard.html"),
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&path, html).with_context(|| format!("writing {}", path.display()))?;
    // Derived from health data → same owner-only posture as the store records.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .with_context(|| format!("securing {}", path.display()))?;
    }
    if open_browser {
        if let Err(e) = open::that(&path) {
            eprintln!(
                "could not open a browser ({e}) — open the file manually: {}",
                path.display()
            );
        }
    }
    Ok(format!("{}\n", path.display()))
}

// ---------------------------------------------------------------------------------------
// Rendered commands
// ---------------------------------------------------------------------------------------

fn fmt_opt(v: Option<f64>, digits: usize) -> String {
    v.map(|x| format!("{x:.digits$}"))
        .unwrap_or_else(|| "-".into())
}

fn fmt_load(load: &WeekLoad) -> String {
    if load.features.is_empty() {
        return "no schedule context".into();
    }
    let mut parts = Vec::new();
    for (feature, value) in &load.features {
        parts.push(format!("{feature} {value:.1}"));
    }
    parts.join(", ")
}

/// `oura capacity`: the one-glance answer plus its full attribution.
pub fn capacity(store: &HealthStore, today: NaiveDate, render: RenderOptions) -> Result<String> {
    let report = fetch_capacity(store, today)?;

    let mut fields: Vec<(&str, String)> = vec![
        (
            "capacity",
            format!("{}% ({})", report.capacity_pct, report.band),
        ),
        ("week of", report.week.week_start.to_string()),
        ("week load", fmt_load(&report.week)),
    ];
    for component in &report.components {
        fields.push((
            component.name,
            format!("-{:.1} pts — {}", component.points, component.detail),
        ));
    }
    for analog in &report.analogs {
        fields.push((
            "analog",
            format!(
                "week of {} (distance {:.2}): readiness during {} → after {}",
                analog.load.week_start,
                analog.distance,
                fmt_opt(analog.during.readiness_mean, 1),
                fmt_opt(analog.after.readiness_mean, 1),
            ),
        ));
    }
    render_record(&report, &fields, render)
}

/// `oura context`: the merged day-grain records — what the engine actually sees.
pub fn context(store: &HealthStore, range: DateRange, render: RenderOptions) -> Result<String> {
    let days = fetch_day_context(store, range)?;

    let mut table = Table::new([
        "DAY", "MTG_H", "EVTS", "EVE", "TRK_H", "READY", "SLEEP", "ACT", "A_SLP_H", "A_HRV",
    ]);
    for (day, record) in &days {
        let cal = record.calendar.as_ref();
        let tog = record.toggl.as_ref();
        let oura = record.oura.as_ref();
        let apple = record.apple.as_ref();
        table.row([
            day.to_string(),
            cal.map(|c| format!("{:.1}", c.meeting_hours))
                .unwrap_or_else(|| "-".into()),
            cal.map(|c| c.event_count.to_string())
                .unwrap_or_else(|| "-".into()),
            cal.map(|c| c.evening_event_count.to_string())
                .unwrap_or_else(|| "-".into()),
            tog.map(|t| format!("{:.1}", t.tracked_hours))
                .unwrap_or_else(|| "-".into()),
            fmt_opt(oura.and_then(|o| o.readiness_score), 0),
            fmt_opt(oura.and_then(|o| o.sleep_score), 0),
            fmt_opt(oura.and_then(|o| o.activity_score), 0),
            fmt_opt(apple.and_then(|a| a.sleep_minutes).map(|m| m / 60.0), 1),
            fmt_opt(apple.and_then(|a| a.hrv_sdnn_ms), 0),
        ]);
    }
    // `--json` serializes the full DayMap (every field, not just the table's columns).
    render_result(&days, &table, render)
}
