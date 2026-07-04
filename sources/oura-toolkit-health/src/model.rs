//! The day-grain data model: one record per LOCAL calendar day, one optional slot per
//! source. This is the normalization layer the vision doc calls for
//! (docs/vision-2026-07-03-personal-health-context.md §3.2): importers each own exactly
//! one slot, and a re-import REPLACES that slot wholesale for the days it covers —
//! source-level idempotency by construction, no cross-source merging ambiguity.
//!
//! Comparability guard: metrics that look alike but are not comparable across providers
//! stay in separate, provider-named fields — Oura HRV is rMSSD, Apple's is SDNN
//! (different statistics over different windows); the schema itself prevents blending
//! them (feasibility doc §4).
//!
//! Privacy by design: context slots hold DERIVED per-day numbers only (hours, counts,
//! clock positions). Event titles, attendees, locations, and entry descriptions are
//! never part of the model, so they can never reach the store, the CLI, or an MCP
//! result.

use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Every stored day, keyed by local calendar date (the store's on-disk shape).
pub type DayMap = BTreeMap<NaiveDate, DayRecord>;

/// One local calendar day: an optional slot per source. Days exist as soon as any
/// source has data for them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DayRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oura: Option<OuraDay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apple: Option<AppleDay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<CalendarDay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toggl: Option<TogglDay>,
}

impl DayRecord {
    /// True when no source has data — such records are dropped rather than stored.
    pub fn is_empty(&self) -> bool {
        self.oura.is_none()
            && self.apple.is_none()
            && self.calendar.is_none()
            && self.toggl.is_none()
    }
}

/// A source that owns one slot of [`DayRecord`]. `upsert` in the store is generic over
/// this, so an importer physically cannot write another source's slot.
pub trait SourceDay: Sized + Clone + PartialEq {
    /// Human-facing source name (used in import summaries).
    const NAME: &'static str;
    /// The slot this source owns.
    fn slot(day: &mut DayRecord) -> &mut Option<Self>;
    /// Read access to the slot.
    fn get(day: &DayRecord) -> Option<&Self>;
}

macro_rules! source_day {
    ($ty:ident, $field:ident, $name:literal) => {
        impl SourceDay for $ty {
            const NAME: &'static str = $name;
            fn slot(day: &mut DayRecord) -> &mut Option<Self> {
                &mut day.$field
            }
            fn get(day: &DayRecord) -> Option<&Self> {
                day.$field.as_ref()
            }
        }
    };
}

/// Daily Oura summaries, written by `oura sync` from the same generated-client data
/// plane the CLI commands render from. Scores are Oura's 0–100 dailies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OuraDay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_score: Option<f64>,
    /// Readiness temperature deviation (°C, signed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_deviation_c: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<f64>,
    /// Daily stress: seconds the day spent in high stress / high recovery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stress_high_seconds: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_high_seconds: Option<f64>,
}
source_day!(OuraDay, oura, "Oura");

/// Daily aggregates from an Apple Health export. Raw samples are aggregated at import
/// and NOT retained (privacy + size: a multi-year export is GBs of samples; the
/// day-grain story needs none of them). Sleep is attributed to the day the sample ENDS
/// (the wake day, matching Oura's daily-sleep convention).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppleDay {
    /// Total minutes asleep (all asleep* stages).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rem_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awake_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resting_hr_bpm: Option<f64>,
    /// Apple reports HRV as SDNN — NOT comparable with Oura's rMSSD; keep provider-tagged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrv_sdnn_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub respiratory_rate: Option<f64>,
    /// Mean blood-oxygen saturation, percent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spo2_pct: Option<f64>,
    /// Sleeping wrist temperature, absolute °C (Apple reports absolute; Oura reports a
    /// deviation — another deliberately unmerged pair).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrist_temp_c: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vo2_max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_energy_kcal: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exercise_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stand_hours: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mindful_minutes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workout_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workout_minutes: Option<f64>,
}
source_day!(AppleDay, apple, "Apple Health");

/// Per-day schedule shape derived from a calendar (.ics) import. Derived numbers only —
/// see the module doc's privacy rule. Times of day are minutes since local midnight.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalendarDay {
    /// Sum of timed-event durations, in hours (all-day events excluded).
    pub meeting_hours: f64,
    /// Count of timed events.
    pub event_count: u32,
    /// Timed events overlapping 18:00–24:00 local.
    pub evening_event_count: u32,
    /// All-day events (kept as a count only; they carry no hours).
    pub all_day_event_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_event_start_min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_event_end_min: Option<u32>,
}
source_day!(CalendarDay, calendar, "calendar");

/// Per-day tracked-time shape from a Toggl Track detailed-report CSV export. Toggl's
/// time blocks are the "how the day was actually spent" counterpart to the calendar's
/// "how the day was planned".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TogglDay {
    /// Sum of tracked entry durations, in hours.
    pub tracked_hours: f64,
    pub entry_count: u32,
    /// Longest single tracked entry, in minutes (a proxy for deep-work block length).
    pub longest_block_minutes: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_start_min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_end_min: Option<u32>,
}
source_day!(TogglDay, toggl, "Toggl");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_record_is_empty_only_with_no_slots() {
        let mut day = DayRecord::default();
        assert!(day.is_empty());
        day.calendar = Some(CalendarDay::default());
        assert!(!day.is_empty());
    }

    #[test]
    fn source_day_slots_are_disjoint() {
        // The trait wiring must route each source to its own slot — a copy-paste slip
        // in the macro invocations would silently cross-write sources.
        let mut day = DayRecord::default();
        *AppleDay::slot(&mut day) = Some(AppleDay::default());
        *TogglDay::slot(&mut day) = Some(TogglDay {
            tracked_hours: 2.0,
            ..Default::default()
        });
        assert!(day.apple.is_some());
        assert!(day.oura.is_none());
        assert!(day.calendar.is_none());
        assert_eq!(TogglDay::get(&day).unwrap().tracked_hours, 2.0);
    }

    #[test]
    fn day_map_round_trips_with_date_string_keys() {
        let mut days = DayMap::new();
        days.insert(
            NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            DayRecord {
                oura: Some(OuraDay {
                    readiness_score: Some(88.0),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
        let json = serde_json::to_string(&days).unwrap();
        assert!(
            json.contains("\"2026-07-01\""),
            "dates serialize as YYYY-MM-DD keys: {json}"
        );
        let back: DayMap = serde_json::from_str(&json).unwrap();
        assert_eq!(back, days);
    }
}
