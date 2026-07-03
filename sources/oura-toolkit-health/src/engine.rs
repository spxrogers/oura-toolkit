//! The analog + capacity engine: deterministic statistics over the day-grain store.
//!
//! Doctrine (vision doc §3.3, non-negotiable): **Rust computes, the assistant
//! narrates.** Everything here is deterministic and unit-tested; the LLM consuming the
//! MCP tools interprets — it never does the arithmetic. This is n=1 observational
//! data, so outputs are phrased as "in your history, weeks like this were followed
//! by X" — never predictions — and the engine REFUSES ([`HealthError::InsufficientHistory`])
//! below [`MIN_HISTORY_WEEKS`] rather than extrapolate from two data points.
//!
//! Week convention: ISO weeks, Monday start ([`week_start`]). Outcome windows trail
//! the context window (the "during" week plus the [`OUTCOME_LAG_WEEKS`] after it):
//! schedule damage often lands the following week.

use std::collections::BTreeMap;

use chrono::{Datelike as _, NaiveDate};
use serde::Serialize;

use crate::error::HealthError;
use crate::model::DayMap;

/// Minimum count of complete past weeks carrying BOTH context and health data before
/// analogs/capacity will answer.
pub const MIN_HISTORY_WEEKS: u32 = 8;

/// How many top analog weeks reports carry by default.
pub const DEFAULT_ANALOG_COUNT: usize = 3;

/// Outcome windows extend this many weeks past the context week.
pub const OUTCOME_LAG_WEEKS: u32 = 2;

/// Context feature names (the load side of a week signature). Stable strings: they are
/// serialized into MCP results and rendered by the CLI.
pub const FEATURE_MEETING_HOURS: &str = "meeting_hours";
pub const FEATURE_EVENT_COUNT: &str = "event_count";
pub const FEATURE_EVENING_EVENTS: &str = "evening_events";
pub const FEATURE_TRACKED_HOURS: &str = "tracked_hours";

/// The Monday of `date`'s ISO week.
pub fn week_start(date: NaiveDate) -> NaiveDate {
    date - chrono::Days::new(u64::from(date.weekday().num_days_from_monday()))
}

/// One week's schedule load: summed context features over its 7 days.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeekLoad {
    pub week_start: NaiveDate,
    /// Feature name → summed value. Only features with at least one contributing day
    /// are present.
    pub features: BTreeMap<String, f64>,
    /// Days of the week that had any context data at all.
    pub days_with_context: u32,
}

/// One window's health outcomes: means over the days that had health data.
/// Provider-tagged (Oura scores vs Apple aggregates) — never blended.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WeekOutcomes {
    pub readiness_mean: Option<f64>,
    pub sleep_score_mean: Option<f64>,
    pub apple_sleep_minutes_mean: Option<f64>,
    pub apple_hrv_sdnn_mean: Option<f64>,
    pub days_with_health: u32,
}

/// A historical week ranked similar to the target, with what followed it.
#[derive(Debug, Clone, Serialize)]
pub struct AnalogWeek {
    /// Z-score euclidean distance from the target week's load (lower = more similar).
    pub distance: f64,
    pub load: WeekLoad,
    /// Outcomes during the analog week itself.
    pub during: WeekOutcomes,
    /// Outcomes over the [`OUTCOME_LAG_WEEKS`] weeks after it.
    pub after: WeekOutcomes,
}

/// The full analog answer for one target week.
#[derive(Debug, Clone, Serialize)]
pub struct AnalogReport {
    /// The week being matched (historical or future — future weeks carry calendar
    /// context from imported .ics files before they happen).
    pub target: WeekLoad,
    /// Personal baseline outcomes across ALL qualifying history (the comparison anchor).
    pub baseline: WeekOutcomes,
    /// Top analogs, most similar first.
    pub analogs: Vec<AnalogWeek>,
    /// How many complete history weeks qualified for matching.
    pub history_weeks: u32,
}

/// Capacity bands, tuned for the CLI's one-glance answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapacityBand {
    /// ≥ 70%: room for more.
    Comfortable,
    /// 40–69%: watch the load.
    Stretched,
    /// < 40%: historically, weeks like this cost you — protect recovery.
    Overloaded,
}

impl CapacityBand {
    pub fn from_pct(pct: u8) -> Self {
        match pct {
            70..=100 => CapacityBand::Comfortable,
            40..=69 => CapacityBand::Stretched,
            _ => CapacityBand::Overloaded,
        }
    }
}

impl std::fmt::Display for CapacityBand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CapacityBand::Comfortable => "comfortable",
            CapacityBand::Stretched => "stretched",
            CapacityBand::Overloaded => "overloaded",
        })
    }
}

/// One deduction in the capacity computation — the transparency layer: every point
/// subtracted from 100 is attributable, so "why 62%?" always has an answer.
#[derive(Debug, Clone, Serialize)]
pub struct CapacityComponent {
    /// Stable component name: `recovery_debt`, `week_load`, or `analog_risk`.
    pub name: &'static str,
    /// Points deducted (≥ 0).
    pub points: f64,
    /// Human-readable one-liner of what drove the deduction.
    pub detail: String,
}

/// The capacity answer: "how much more can this week take?"
#[derive(Debug, Clone, Serialize)]
pub struct CapacityReport {
    pub as_of: NaiveDate,
    /// 0–100: 100 = fully recovered and an unloaded week; the deductions in
    /// `components` explain the rest.
    pub capacity_pct: u8,
    pub band: CapacityBand,
    pub components: Vec<CapacityComponent>,
    /// The current week's load (the week containing `as_of`).
    pub week: WeekLoad,
    /// Analogs of the current week, when history supports them.
    pub analogs: Vec<AnalogWeek>,
}

/// Rank the historical weeks most similar to `target_week` (its Monday) by schedule
/// load, and report the health outcomes during and after each.
///
/// Errors with [`HealthError::InsufficientHistory`] below [`MIN_HISTORY_WEEKS`]
/// qualifying weeks (a week qualifies when it is complete, in the past, and has both
/// context and health data).
pub fn find_analogs(
    days: &DayMap,
    target_week: NaiveDate,
    k: usize,
) -> Result<AnalogReport, HealthError> {
    let _ = (days, target_week, k);
    unimplemented!("Agent C: analog matching")
}

/// The capacity score for the week containing `today`. See [`CapacityReport`].
pub fn capacity(days: &DayMap, today: NaiveDate) -> Result<CapacityReport, HealthError> {
    let _ = (days, today);
    unimplemented!("Agent C: capacity")
}

/// Schedule load for the next `weeks` weeks starting from the week containing `today`
/// (inclusive). Future context exists because .ics imports carry future events. Weeks
/// with no context at all are still listed (empty features) so gaps are visible.
pub fn upcoming_load(days: &DayMap, today: NaiveDate, weeks: u32) -> Vec<WeekLoad> {
    let _ = (days, today, weeks);
    unimplemented!("Agent C: upcoming load")
}
