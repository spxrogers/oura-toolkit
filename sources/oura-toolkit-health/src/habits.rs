//! Habit statistics: boolean per-day logs read at the LONG grain.
//!
//! The design premise (owner-specified): consistency happens over weeks, not days —
//! so the primary read is a **moving-average rate in days-per-week over trailing
//! windows** (7/28/91 days), not a daily checkbox or a streak. "Am I strength
//! training 4×/week? Meditating 6×/week?" is answered by `rate_28d`, and drift shows
//! up as `rate_7d` pulling away from `rate_91d`.
//!
//! Same doctrine as the engine: deterministic, no clock reads (`today` injected),
//! provider-free (habits are the user's own logs).

use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::Serialize;

use crate::error::HealthError;
use crate::model::DayMap;

/// The trailing windows the rates are computed over, in days.
pub const RATE_WINDOWS: [u32; 3] = [7, 28, 91];

/// Maximum normalized habit-name length.
pub const MAX_HABIT_NAME: usize = 64;

/// Canonicalize a habit name so "Strength Training", "strength  training" and
/// "strength-training" are ONE habit: trim, lowercase, collapse every run of
/// whitespace/underscores/hyphens to a single `-`, keep only ASCII alphanumerics and
/// `-`. Rejects names that end up empty or longer than [`MAX_HABIT_NAME`] — including
/// anything made only of control bytes or symbols (a hostile name cannot survive into
/// the store, the CLI, an MCP result, or the dashboard).
pub fn normalize_habit_name(raw: &str) -> Result<String, HealthError> {
    let mut out = String::new();
    let mut pending_sep = false;
    for c in raw.trim().chars() {
        if c.is_ascii_alphanumeric() {
            if pending_sep && !out.is_empty() {
                out.push('-');
            }
            pending_sep = false;
            out.push(c.to_ascii_lowercase());
        } else if c.is_whitespace() || c == '-' || c == '_' {
            pending_sep = true;
        }
        // Every other character (punctuation, control bytes, emoji) is dropped.
    }
    if out.is_empty() {
        return Err(HealthError::InvalidHabitName {
            reason: "no letters or digits left after normalization".to_string(),
        });
    }
    if out.len() > MAX_HABIT_NAME {
        return Err(HealthError::InvalidHabitName {
            reason: format!("longer than {MAX_HABIT_NAME} characters"),
        });
    }
    Ok(out)
}

/// One habit's long-grain read. Rates are **days per week** over the trailing window
/// ending at `today` (inclusive); each window is clamped to the period the habit has
/// actually been tracked (from its first log), so a habit started 10 days ago is not
/// diluted by 81 days of "before it existed".
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HabitStats {
    pub name: String,
    pub first_logged: NaiveDate,
    pub last_logged: NaiveDate,
    /// Total days ever logged done.
    pub total_days: u32,
    /// Days/week over the trailing 7 days (this week's raw shape).
    pub rate_7d: f64,
    /// Days/week over the trailing 28 days — the headline consistency number.
    pub rate_28d: f64,
    /// Days/week over the trailing 91 days (the long tail).
    pub rate_91d: f64,
    /// How many days the effective 91-day window actually covered (window vs
    /// first-log clamp made visible, so a young habit's rates read honestly).
    pub days_tracked_91d: u32,
}

/// Days-per-week rate over the trailing `window` days ending at `today`, clamped to
/// the habit's tracked period.
fn rate(done_dates: &[NaiveDate], first: NaiveDate, today: NaiveDate, window: u32) -> (f64, u32) {
    let window_start = today - chrono::Days::new(u64::from(window) - 1);
    let effective_start = window_start.max(first);
    if effective_start > today {
        return (0.0, 0);
    }
    let effective_days = (today - effective_start).num_days() as u32 + 1;
    let done = done_dates
        .iter()
        .filter(|d| **d >= effective_start && **d <= today)
        .count() as f64;
    (done / f64::from(effective_days) * 7.0, effective_days)
}

/// Every habit's stats as of `today`, sorted by name. Habits are discovered from the
/// store itself — there is no separate registry to fall out of sync with.
pub fn habit_stats(days: &DayMap, today: NaiveDate) -> Vec<HabitStats> {
    let mut by_name: BTreeMap<&str, Vec<NaiveDate>> = BTreeMap::new();
    for (date, rec) in days {
        if let Some(h) = &rec.habits {
            for name in &h.done {
                by_name.entry(name).or_default().push(*date);
            }
        }
    }
    by_name
        .into_iter()
        .map(|(name, dates)| {
            // DayMap iteration is date-ordered, so `dates` is sorted.
            let first = dates[0];
            let last = *dates.last().expect("non-empty by construction");
            let (rate_7d, _) = rate(&dates, first, today, RATE_WINDOWS[0]);
            let (rate_28d, _) = rate(&dates, first, today, RATE_WINDOWS[1]);
            let (rate_91d, days_tracked_91d) = rate(&dates, first, today, RATE_WINDOWS[2]);
            HabitStats {
                name: name.to_string(),
                first_logged: first,
                last_logged: last,
                total_days: dates.len() as u32,
                rate_7d,
                rate_28d,
                rate_91d,
                days_tracked_91d,
            }
        })
        .collect()
}

/// The dashboard's chart series: for each day from the habit's first log through
/// `today`, the trailing-`window`-day rate (days/week, first-log clamped) as of that
/// day — the habit's consistency as a line, not a row of checkboxes.
pub fn habit_rate_series(
    days: &DayMap,
    name: &str,
    today: NaiveDate,
    window: u32,
) -> Vec<(NaiveDate, f64)> {
    let done_dates: Vec<NaiveDate> = days
        .iter()
        .filter(|(_, rec)| rec.habits.as_ref().is_some_and(|h| h.done.contains(name)))
        .map(|(d, _)| *d)
        .collect();
    let Some(&first) = done_dates.first() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut day = first;
    while day <= today {
        out.push((day, rate(&done_dates, first, day, window).0));
        day = day.succ_opt().expect("date within chrono range");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HabitsDay;

    fn d(s: &str) -> NaiveDate {
        s.parse().expect("valid date")
    }

    fn store_with(logged: &[(&str, &[&str])]) -> DayMap {
        // (habit, dates) pairs → DayMap
        let mut days = DayMap::new();
        for (habit, dates) in logged {
            for date in *dates {
                days.entry(d(date))
                    .or_default()
                    .habits
                    .get_or_insert_with(HabitsDay::default)
                    .done
                    .insert((*habit).to_string());
            }
        }
        days
    }

    #[test]
    fn names_normalize_to_one_canonical_habit() {
        for raw in [
            "Strength Training",
            "strength-training",
            "  strength_training  ",
            "STRENGTH   TRAINING",
        ] {
            assert_eq!(
                normalize_habit_name(raw).unwrap(),
                "strength-training",
                "{raw:?} must canonicalize"
            );
        }
    }

    #[test]
    fn hostile_names_are_rejected_or_fully_sanitized() {
        // Names with nothing alphanumeric must be rejected, not stored as empty.
        for raw in ["", "   ", "\x1b\x07", "!!!", "\u{202e}"] {
            let err = normalize_habit_name(raw).unwrap_err();
            assert!(
                matches!(err, HealthError::InvalidHabitName { .. }),
                "{raw:?} must be invalid, got {err:?}"
            );
        }
        // Hostile input keeps ONLY alphanumerics: escape sequences lose their ESC (the
        // digits/letters that remain are harmless), tags lose their brackets.
        assert_eq!(normalize_habit_name("\x1b[2J").unwrap(), "2j");
        let n = normalize_habit_name("med\x1b[2Jitate<script>").unwrap();
        assert_eq!(n, "med2jitatescript", "no control bytes or <> survive");
        assert!(
            !n.chars().any(|c| c.is_control() || c == '<' || c == '>'),
            "sanitized output is inert everywhere it is rendered"
        );
        // Length cap.
        let long = "a".repeat(MAX_HABIT_NAME + 1);
        assert!(normalize_habit_name(&long).is_err());
    }

    #[test]
    fn rates_are_days_per_week_over_trailing_windows() {
        // 4 of the last 7 days, 12 of the last 28, habit older than 91 days.
        let today = d("2026-07-03");
        let mut dates: Vec<&str> = vec!["2026-06-27", "2026-06-29", "2026-07-01", "2026-07-03"];
        // 8 more inside the 28-day window but outside the last 7 (June 6..26).
        dates.extend([
            "2026-06-06",
            "2026-06-08",
            "2026-06-10",
            "2026-06-12",
            "2026-06-14",
            "2026-06-16",
            "2026-06-18",
            "2026-06-20",
        ]);
        // One old log so first_logged is > 91 days back (2026-04-01).
        dates.push("2026-04-01");
        let days = store_with(&[("exercise", &dates)]);
        let stats = habit_stats(&days, today);
        assert_eq!(stats.len(), 1);
        let s = &stats[0];
        assert_eq!(s.name, "exercise");
        assert_eq!(s.total_days, 13);
        assert!(
            (s.rate_7d - 4.0).abs() < 1e-9,
            "4/7d → 4.0×/wk: {}",
            s.rate_7d
        );
        assert!(
            (s.rate_28d - 3.0).abs() < 1e-9,
            "12/28d → 3.0×/wk: {}",
            s.rate_28d
        );
        assert_eq!(s.days_tracked_91d, 91, "old habit → full window");
        assert!(
            (s.rate_91d - (13.0 - 1.0) / 91.0 * 7.0).abs() < 1e-9,
            "12 logs inside the 91d window (the April log is outside): {}",
            s.rate_91d
        );
        assert_eq!(s.first_logged, d("2026-04-01"));
        assert_eq!(s.last_logged, d("2026-07-03"));
    }

    #[test]
    fn young_habits_clamp_the_window_to_their_tracked_period() {
        // Logged 5 of the 10 days since first log; a naive /91 would read 0.38×/wk.
        let today = d("2026-07-03");
        let days = store_with(&[(
            "meditate",
            &[
                "2026-06-24",
                "2026-06-26",
                "2026-06-28",
                "2026-06-30",
                "2026-07-02",
            ],
        )]);
        let s = &habit_stats(&days, today)[0];
        assert_eq!(s.days_tracked_91d, 10, "tracked-period clamp");
        assert!(
            (s.rate_91d - 3.5).abs() < 1e-9,
            "5/10 days → 3.5×/wk, not 5/91: {}",
            s.rate_91d
        );
    }

    #[test]
    fn multiple_habits_are_independent_and_sorted() {
        let today = d("2026-07-03");
        let days = store_with(&[
            ("meditate", &["2026-07-01", "2026-07-02", "2026-07-03"]),
            ("exercise", &["2026-07-02"]),
        ]);
        let stats = habit_stats(&days, today);
        let names: Vec<&str> = stats.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["exercise", "meditate"], "sorted by name");
        assert_eq!(stats[0].total_days, 1);
        assert_eq!(stats[1].total_days, 3);
    }

    #[test]
    fn rate_series_walks_daily_from_first_log() {
        let today = d("2026-07-03");
        let days = store_with(&[("exercise", &["2026-07-01", "2026-07-03"])]);
        let series = habit_rate_series(&days, "exercise", today, 28);
        assert_eq!(series.len(), 3, "first log through today, daily");
        assert_eq!(series[0].0, d("2026-07-01"));
        // Day 1: 1 done / 1 tracked day → 7.0×/wk (clamped window).
        assert!((series[0].1 - 7.0).abs() < 1e-9);
        // Day 2: 1/2 → 3.5. Day 3: 2/3 → 4.666…
        assert!((series[1].1 - 3.5).abs() < 1e-9);
        assert!((series[2].1 - (2.0 / 3.0 * 7.0)).abs() < 1e-9);
        assert!(
            habit_rate_series(&days, "never-logged", today, 28).is_empty(),
            "unknown habit → empty series"
        );
    }
}
