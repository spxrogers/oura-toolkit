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

use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike as _, Days, NaiveDate};
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

/// Rank the historical weeks most similar to `target_week` (normalized to its Monday)
/// by schedule load, and report the health outcomes during and after each.
///
/// A week is a candidate when: it AND its [`OUTCOME_LAG_WEEKS`]-week after-window end
/// on or before the target week's Monday (so "what followed" exists in time — the
/// after-window may still hold zero health days, and is reported as-is), it has at
/// least one day with schedule context, and at least one day with health data during
/// the week itself. Errors with [`HealthError::InsufficientHistory`] when fewer than
/// [`MIN_HISTORY_WEEKS`] weeks qualify.
pub fn find_analogs(
    days: &DayMap,
    target_week: NaiveDate,
    k: usize,
) -> Result<AnalogReport, HealthError> {
    analog_core(days, week_start(target_week), k).map(|(report, _)| report)
}

/// The capacity score for the week containing `today`. See [`CapacityReport`].
///
/// Errors with [`HealthError::InsufficientHistory`] exactly as [`find_analogs`] does
/// for the current week: a capacity number without enough history to anchor it would
/// be fabricated confidence.
pub fn capacity(days: &DayMap, today: NaiveDate) -> Result<CapacityReport, HealthError> {
    // History gate first: everything below leans on the analog machinery.
    let (report, load_z) = analog_core(days, week_start(today), DEFAULT_ANALOG_COUNT)?;

    let recovery = recovery_debt_component(days, today);

    // Week load: how far this week's schedule sits above the personal norm.
    // points = max(aggregate z, 0) × 12, capped at 35; z is the mean of the
    // per-feature z-scores over the same normalization find_analogs used.
    let points = (load_z.max(0.0) * 12.0).min(35.0);
    let week_load = CapacityComponent {
        name: "week_load",
        points,
        detail: format!(
            "week load z-score {load_z:+.1} vs {} history weeks",
            report.history_weeks
        ),
    };

    let analog_risk = analog_risk_component(&report);

    let components = vec![recovery, week_load, analog_risk];
    let deducted: f64 = components.iter().map(|c| c.points).sum();
    let capacity_pct = (100.0 - deducted).round().clamp(0.0, 100.0) as u8;

    Ok(CapacityReport {
        as_of: today,
        capacity_pct,
        band: CapacityBand::from_pct(capacity_pct),
        components,
        week: report.target,
        analogs: report.analogs,
    })
}

/// Schedule load for the next `weeks` weeks starting from the week containing `today`
/// (inclusive). Future context exists because .ics imports carry future events. Weeks
/// with no context at all are still listed (empty features) so gaps are visible.
pub fn upcoming_load(days: &DayMap, today: NaiveDate, weeks: u32) -> Vec<WeekLoad> {
    let start = week_start(today);
    (0..weeks)
        .map(|i| compute_week_load(days, add_weeks(start, i)))
        .collect()
}

// ---------------------------------------------------------------------------
// Private engine internals
// ---------------------------------------------------------------------------

fn add_weeks(date: NaiveDate, weeks: u32) -> NaiveDate {
    date + Days::new(u64::from(weeks) * 7)
}

/// One week's summed context features (the week signature). A feature appears only if
/// at least one day of the week carries its source's slot; `days_with_context` counts
/// days with calendar OR toggl data.
fn compute_week_load(days: &DayMap, ws: NaiveDate) -> WeekLoad {
    let end = add_weeks(ws, 1);
    let mut cal_present = false;
    let mut toggl_present = false;
    let (mut meeting_hours, mut event_count, mut evening_events, mut tracked_hours) =
        (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut days_with_context = 0u32;
    for (_, rec) in days.range(ws..end) {
        if rec.calendar.is_some() || rec.toggl.is_some() {
            days_with_context += 1;
        }
        if let Some(cal) = &rec.calendar {
            cal_present = true;
            meeting_hours += cal.meeting_hours;
            event_count += f64::from(cal.event_count);
            evening_events += f64::from(cal.evening_event_count);
        }
        if let Some(toggl) = &rec.toggl {
            toggl_present = true;
            tracked_hours += toggl.tracked_hours;
        }
    }
    let mut features = BTreeMap::new();
    if cal_present {
        features.insert(FEATURE_MEETING_HOURS.to_string(), meeting_hours);
        features.insert(FEATURE_EVENT_COUNT.to_string(), event_count);
        features.insert(FEATURE_EVENING_EVENTS.to_string(), evening_events);
    }
    if toggl_present {
        features.insert(FEATURE_TRACKED_HOURS.to_string(), tracked_hours);
    }
    WeekLoad {
        week_start: ws,
        features,
        days_with_context,
    }
}

/// Accumulates provider-tagged outcome means over a set of days. Oura scores and Apple
/// aggregates are summed into separate fields — never blended into one number.
#[derive(Default)]
struct OutcomeAcc {
    readiness_sum: f64,
    readiness_n: u32,
    sleep_score_sum: f64,
    sleep_score_n: u32,
    apple_sleep_sum: f64,
    apple_sleep_n: u32,
    apple_hrv_sum: f64,
    apple_hrv_n: u32,
    days_with_health: u32,
}

impl OutcomeAcc {
    fn add_day(&mut self, rec: &crate::model::DayRecord) {
        let mut has_health = false;
        if let Some(oura) = &rec.oura {
            has_health = true;
            if let Some(v) = oura.readiness_score {
                self.readiness_sum += v;
                self.readiness_n += 1;
            }
            if let Some(v) = oura.sleep_score {
                self.sleep_score_sum += v;
                self.sleep_score_n += 1;
            }
        }
        if let Some(apple) = &rec.apple {
            has_health = true;
            if let Some(v) = apple.sleep_minutes {
                self.apple_sleep_sum += v;
                self.apple_sleep_n += 1;
            }
            if let Some(v) = apple.hrv_sdnn_ms {
                self.apple_hrv_sum += v;
                self.apple_hrv_n += 1;
            }
        }
        if has_health {
            self.days_with_health += 1;
        }
    }

    fn finish(self) -> WeekOutcomes {
        fn mean(sum: f64, n: u32) -> Option<f64> {
            if n > 0 {
                Some(sum / f64::from(n))
            } else {
                None
            }
        }
        WeekOutcomes {
            readiness_mean: mean(self.readiness_sum, self.readiness_n),
            sleep_score_mean: mean(self.sleep_score_sum, self.sleep_score_n),
            apple_sleep_minutes_mean: mean(self.apple_sleep_sum, self.apple_sleep_n),
            apple_hrv_sdnn_mean: mean(self.apple_hrv_sum, self.apple_hrv_n),
            days_with_health: self.days_with_health,
        }
    }
}

/// Outcome means over the `len_days` days starting at `start`.
fn window_outcomes(days: &DayMap, start: NaiveDate, len_days: u64) -> WeekOutcomes {
    let end = start + Days::new(len_days);
    let mut acc = OutcomeAcc::default();
    for (_, rec) in days.range(start..end) {
        acc.add_day(rec);
    }
    acc.finish()
}

/// A week's raw value for one feature. A week lacking the feature counts as 0.0: the
/// context sources are all-or-nothing per day, so absence means nothing was planned or
/// tracked — an un-tracked week IS a light week, not missing data.
fn feature_value(load: &WeekLoad, name: &str) -> f64 {
    load.features.get(name).copied().unwrap_or(0.0)
}

struct CandidateWeek {
    load: WeekLoad,
    during: WeekOutcomes,
    after: WeekOutcomes,
}

/// The shared core behind [`find_analogs`] and [`capacity`]: the analog report for the
/// week starting at `target_ws` (must already be a Monday) plus the target week's
/// aggregate load z-score (mean of per-feature z-scores) against the candidate weeks.
fn analog_core(
    days: &DayMap,
    target_ws: NaiveDate,
    k: usize,
) -> Result<(AnalogReport, f64), HealthError> {
    let target = compute_week_load(days, target_ws);

    // Candidate weeks: distinct Monday-start weeks whose after-window ends on or
    // before the target week (so "what followed" exists in time), with ≥1 context day
    // and ≥1 health day during the week itself. The after-window itself may be empty
    // of health data — what's there is reported as-is.
    let week_starts: BTreeSet<NaiveDate> = days
        .keys()
        .map(|d| week_start(*d))
        .filter(|ws| add_weeks(*ws, 1 + OUTCOME_LAG_WEEKS) <= target_ws)
        .collect();
    let mut candidates = Vec::new();
    for ws in week_starts {
        let load = compute_week_load(days, ws);
        if load.days_with_context == 0 {
            continue;
        }
        let during = window_outcomes(days, ws, 7);
        if during.days_with_health == 0 {
            continue;
        }
        let after = window_outcomes(days, add_weeks(ws, 1), u64::from(OUTCOME_LAG_WEEKS) * 7);
        candidates.push(CandidateWeek {
            load,
            during,
            after,
        });
    }

    let have = candidates.len() as u32;
    if have < MIN_HISTORY_WEEKS {
        return Err(HealthError::InsufficientHistory {
            needed: MIN_HISTORY_WEEKS,
            have,
        });
    }

    // NOTE: a target week with no context at all (days_with_context == 0) is NOT an
    // error — an empty planned week is legitimately "the lightest week", and it is
    // matched as an all-zero signature. The only refusal here is thin history.

    // Normalization: per-feature mean and population std across the CANDIDATE weeks,
    // over the union of features present in the target or any candidate. std == 0
    // means the feature carries no signal — it contributes zero distance and zero z.
    let mut names: BTreeSet<String> = target.features.keys().cloned().collect();
    for cand in &candidates {
        names.extend(cand.load.features.keys().cloned());
    }
    let n = candidates.len() as f64;
    let stats: Vec<(String, f64, f64)> = names
        .into_iter()
        .map(|name| {
            let mean = candidates
                .iter()
                .map(|c| feature_value(&c.load, &name))
                .sum::<f64>()
                / n;
            let variance = candidates
                .iter()
                .map(|c| (feature_value(&c.load, &name) - mean).powi(2))
                .sum::<f64>()
                / n;
            (name, mean, variance.sqrt())
        })
        .collect();

    // Aggregate z of the target week (for capacity's week_load component).
    let load_z = if stats.is_empty() {
        0.0
    } else {
        stats
            .iter()
            .map(|(name, mean, std)| {
                if *std == 0.0 {
                    0.0
                } else {
                    (feature_value(&target, name) - mean) / std
                }
            })
            .sum::<f64>()
            / stats.len() as f64
    };

    // Baseline: outcomes over ALL days belonging to any candidate week (during-windows
    // only — candidate weeks are distinct Mondays, so days are never double-counted).
    let mut baseline_acc = OutcomeAcc::default();
    for cand in &candidates {
        for (_, rec) in days.range(cand.load.week_start..add_weeks(cand.load.week_start, 1)) {
            baseline_acc.add_day(rec);
        }
    }
    let baseline = baseline_acc.finish();

    // Distance: euclidean over per-feature z-scores; z_cand − z_target reduces to
    // (raw_cand − raw_target) / std.
    let mut scored: Vec<(f64, CandidateWeek)> = candidates
        .into_iter()
        .map(|cand| {
            let d2 = stats
                .iter()
                .map(|(name, _, std)| {
                    if *std == 0.0 {
                        0.0
                    } else {
                        ((feature_value(&cand.load, name) - feature_value(&target, name)) / std)
                            .powi(2)
                    }
                })
                .sum::<f64>();
            (d2.sqrt(), cand)
        })
        .collect();
    // Most similar first; ties break to the NEWER week (more recent = more relevant).
    scored.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| b.1.load.week_start.cmp(&a.1.load.week_start))
    });

    let analogs = scored
        .into_iter()
        .take(k)
        .map(|(distance, cand)| AnalogWeek {
            distance,
            load: cand.load,
            during: cand.during,
            after: cand.after,
        })
        .collect();

    Ok((
        AnalogReport {
            target,
            baseline,
            analogs,
            history_weeks: have,
        },
        load_z,
    ))
}

/// Capacity component 1: how far recovery has fallen below its own baseline.
/// Oura readiness when available; Apple sleep minutes otherwise; 0 points (with an
/// explanatory detail) when neither provider has both a recent and a baseline value.
fn recovery_debt_component(days: &DayMap, today: NaiveDate) -> CapacityComponent {
    let recent_start = today - Days::new(6);

    // Oura path: mean readiness over the last 7 days vs mean over all prior days.
    let recent = mean_metric(days.range(recent_start..=today), |rec| {
        rec.oura.as_ref().and_then(|o| o.readiness_score)
    });
    let baseline = mean_metric(days.range(..recent_start), |rec| {
        rec.oura.as_ref().and_then(|o| o.readiness_score)
    });
    if let (Some(recent), Some(baseline)) = (recent, baseline) {
        // debt = clamp(baseline − recent, 0, 25) × 1.2 → max 30 points.
        let points = (baseline - recent).clamp(0.0, 25.0) * 1.2;
        return CapacityComponent {
            name: "recovery_debt",
            points,
            detail: format!("readiness last 7d {recent:.1} vs baseline {baseline:.1}"),
        };
    }

    // Apple fallback: sleep-minutes deficit, divided by 15 so every 15 min/night of
    // sleep debt ≈ 1 pre-scale point (a lost hour/night ≈ 4), then the same clamp and
    // ×1.2 scale as the readiness path.
    let recent = mean_metric(days.range(recent_start..=today), |rec| {
        rec.apple.as_ref().and_then(|a| a.sleep_minutes)
    });
    let baseline = mean_metric(days.range(..recent_start), |rec| {
        rec.apple.as_ref().and_then(|a| a.sleep_minutes)
    });
    if let (Some(recent), Some(baseline)) = (recent, baseline) {
        let points = ((baseline - recent) / 15.0).clamp(0.0, 25.0) * 1.2;
        return CapacityComponent {
            name: "recovery_debt",
            points,
            detail: format!("Apple sleep last 7d {recent:.1} min/night vs baseline {baseline:.1}"),
        };
    }

    CapacityComponent {
        name: "recovery_debt",
        points: 0.0,
        detail: "no recent recovery data".to_string(),
    }
}

/// Mean of `metric` over the day records yielded by `iter`; `None` when no day has it.
fn mean_metric<'a, I, F>(iter: I, metric: F) -> Option<f64>
where
    I: Iterator<Item = (&'a NaiveDate, &'a crate::model::DayRecord)>,
    F: Fn(&crate::model::DayRecord) -> Option<f64>,
{
    let mut sum = 0.0;
    let mut n = 0u32;
    for (_, rec) in iter {
        if let Some(v) = metric(rec) {
            sum += v;
            n += 1;
        }
    }
    if n > 0 {
        Some(sum / f64::from(n))
    } else {
        None
    }
}

/// Capacity component 3: what followed weeks like this one, as a mean dip below the
/// personal baseline. Per analog: the readiness dip when both the baseline and the
/// analog's after-window carry readiness; else the Apple sleep-minutes dip / 15 (the
/// same pre-scale unit as recovery_debt's fallback); else the analog is skipped.
fn analog_risk_component(report: &AnalogReport) -> CapacityComponent {
    let mut dips = Vec::new();
    for analog in &report.analogs {
        if let (Some(base), Some(after)) =
            (report.baseline.readiness_mean, analog.after.readiness_mean)
        {
            dips.push((base - after).max(0.0));
        } else if let (Some(base), Some(after)) = (
            report.baseline.apple_sleep_minutes_mean,
            analog.after.apple_sleep_minutes_mean,
        ) {
            dips.push(((base - after) / 15.0).max(0.0));
        }
    }
    if dips.is_empty() {
        return CapacityComponent {
            name: "analog_risk",
            points: 0.0,
            detail: "outcomes after similar weeks were unavailable".to_string(),
        };
    }
    let mean_dip = dips.iter().sum::<f64>() / dips.len() as f64;
    CapacityComponent {
        name: "analog_risk",
        points: mean_dip.clamp(0.0, 20.0),
        detail: format!(
            "mean recovery dip {:.1} in the weeks after {} similar past weeks",
            mean_dip,
            dips.len()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AppleDay, CalendarDay, OuraDay, TogglDay};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    /// Test-data builder: each method writes one source slot (or one field of it) for
    /// one day, so fixtures read as a schedule.
    #[derive(Default)]
    struct Fixture {
        days: DayMap,
    }

    impl Fixture {
        fn cal(mut self, date: NaiveDate, meeting_hours: f64, events: u32, evening: u32) -> Self {
            self.days.entry(date).or_default().calendar = Some(CalendarDay {
                meeting_hours,
                event_count: events,
                evening_event_count: evening,
                ..Default::default()
            });
            self
        }

        fn toggl(mut self, date: NaiveDate, tracked_hours: f64) -> Self {
            self.days.entry(date).or_default().toggl = Some(TogglDay {
                tracked_hours,
                ..Default::default()
            });
            self
        }

        fn readiness(mut self, date: NaiveDate, score: f64) -> Self {
            self.days
                .entry(date)
                .or_default()
                .oura
                .get_or_insert_with(OuraDay::default)
                .readiness_score = Some(score);
            self
        }

        fn apple_sleep(mut self, date: NaiveDate, minutes: f64) -> Self {
            self.days
                .entry(date)
                .or_default()
                .apple
                .get_or_insert_with(AppleDay::default)
                .sleep_minutes = Some(minutes);
            self
        }
    }

    /// Ten Monday-anchored history weeks (2026-03-16 .. 2026-05-18): week i carries
    /// `hours[i]` meeting hours and readiness `readiness[i]`, both on its Monday.
    fn gradient_history(hours: &[f64; 10], readiness: &[f64; 10]) -> Fixture {
        let first = d(2026, 3, 16);
        let mut fx = Fixture::default();
        for (i, (&h, &r)) in hours.iter().zip(readiness.iter()).enumerate() {
            let monday = first + Days::new(7 * i as u64);
            fx = fx.cal(monday, h, 0, 0).readiness(monday, r);
        }
        fx
    }

    #[test]
    fn week_start_normalizes_monday_sunday_and_midweek() {
        let monday = d(2026, 6, 29);
        assert_eq!(week_start(monday), monday, "a Monday is its own week start");
        assert_eq!(week_start(d(2026, 7, 1)), monday, "Wednesday → its Monday");
        assert_eq!(
            week_start(d(2026, 7, 5)),
            monday,
            "Sunday belongs to the week of the PRECEDING Monday (ISO weeks)"
        );
    }

    #[test]
    fn find_analogs_ranks_by_load_similarity_with_exact_outcomes() {
        let hours = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let readiness = [60.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0];
        let fx = gradient_history(&hours, &readiness)
            // Health-only weeks after the last candidate: they feed the top analogs'
            // after-windows but have NO context, so they are not candidates and do not
            // enter the baseline.
            .readiness(d(2026, 5, 25), 80.0)
            .readiness(d(2026, 6, 1), 70.0)
            // Planned target week: 95 meeting hours.
            .cal(d(2026, 6, 29), 95.0, 0, 0);

        // Mid-week target date normalizes to its Monday.
        let report = find_analogs(&fx.days, d(2026, 7, 1), 3).unwrap();

        assert_eq!(report.history_weeks, 10);
        assert_eq!(report.target.week_start, d(2026, 6, 29));
        assert_eq!(report.target.features[FEATURE_MEETING_HOURS], 95.0);
        // Baseline = mean readiness over ALL candidate during-days: (60+..+69)/10.
        assert_eq!(report.baseline.readiness_mean, Some(64.5));
        assert_eq!(report.baseline.days_with_health, 10);

        // 100h and 90h are equidistant from 95h → the tie breaks to the NEWER week.
        assert_eq!(report.analogs.len(), 3);
        assert_eq!(report.analogs[0].load.week_start, d(2026, 5, 18));
        assert_eq!(report.analogs[1].load.week_start, d(2026, 5, 11));
        assert_eq!(report.analogs[2].load.week_start, d(2026, 5, 4));

        // Hours 10..100: mean 55, population variance 825 → distance = 5/√825.
        let expected = 5.0 / 825.0_f64.sqrt();
        assert!(
            (report.analogs[0].distance - expected).abs() < 1e-12,
            "distance must be the z-score euclidean over candidate stats: {} vs {expected}",
            report.analogs[0].distance
        );
        assert_eq!(report.analogs[0].distance, report.analogs[1].distance);
        assert!(report.analogs[1].distance < report.analogs[2].distance);

        // Exact during/after means for the top analog (week of 5/18, readiness 69;
        // its after-window 5/25..6/8 holds the 80 and 70 health-only days).
        assert_eq!(report.analogs[0].during.readiness_mean, Some(69.0));
        assert_eq!(report.analogs[0].during.days_with_health, 1);
        assert_eq!(report.analogs[0].after.readiness_mean, Some(75.0));
        assert_eq!(report.analogs[0].after.days_with_health, 2);
        // 5/11's after-window 5/18..6/1 → (69 + 80) / 2.
        assert_eq!(report.analogs[1].after.readiness_mean, Some(74.5));
        // 5/4's after-window 5/11..5/25 → (68 + 69) / 2.
        assert_eq!(report.analogs[2].after.readiness_mean, Some(68.5));
    }

    #[test]
    fn after_window_captures_the_crash_that_follows_a_heavy_week() {
        // THE product claim: schedule damage lands the FOLLOWING weeks. The heavy week
        // itself looks fine during (readiness 80) — the two weeks after it crashed.
        let hours = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let readiness = [70.0, 70.0, 70.0, 70.0, 70.0, 70.0, 70.0, 80.0, 50.0, 50.0];
        let fx = gradient_history(&hours, &readiness).cal(d(2026, 6, 29), 80.0, 0, 0);

        let report = find_analogs(&fx.days, d(2026, 6, 29), 1).unwrap();
        let top = &report.analogs[0];
        assert_eq!(
            top.load.week_start,
            d(2026, 5, 4),
            "80h week is the exact match"
        );
        assert_eq!(top.distance, 0.0);
        assert_eq!(
            top.during.readiness_mean,
            Some(80.0),
            "the heavy week looked FINE while it happened"
        );
        assert_eq!(
            top.after.readiness_mean,
            Some(50.0),
            "the crash must show up in the after-window, not the during-window"
        );
        assert_eq!(top.after.days_with_health, 2);
    }

    #[test]
    fn insufficient_history_reports_needed_and_have() {
        // 7 fully qualifying weeks + 1 context-only week (no health data) that must
        // NOT be counted as a candidate.
        let first = d(2026, 4, 6);
        let mut fx = Fixture::default().cal(d(2026, 3, 30), 5.0, 0, 0);
        for i in 0..7u64 {
            let monday = first + Days::new(7 * i);
            fx = fx.cal(monday, 10.0, 0, 0).readiness(monday, 70.0);
        }
        let err = find_analogs(&fx.days, d(2026, 6, 29), 3).unwrap_err();
        match err {
            HealthError::InsufficientHistory { needed, have } => {
                assert_eq!(needed, MIN_HISTORY_WEEKS);
                assert_eq!(needed, 8);
                assert_eq!(have, 7, "the context-only week must not count as history");
            }
            other => panic!("expected InsufficientHistory, got: {other}"),
        }
    }

    #[test]
    fn apple_only_history_never_synthesizes_readiness() {
        // Provider separation: Apple sleep minutes must never blend into the Oura
        // readiness/sleep-score fields.
        let first = d(2026, 3, 16);
        let mut fx = Fixture::default();
        for i in 0..10u64 {
            let monday = first + Days::new(7 * i);
            #[allow(clippy::cast_precision_loss)]
            let idx = i as f64;
            fx = fx
                .cal(monday, 10.0 * (idx + 1.0), 0, 0)
                .apple_sleep(monday, 400.0 + idx);
        }
        let fx = fx.cal(d(2026, 6, 29), 95.0, 0, 0);

        let report = find_analogs(&fx.days, d(2026, 7, 1), 3).unwrap();
        assert_eq!(
            report.baseline.readiness_mean, None,
            "Oura readiness must never be synthesized from Apple data"
        );
        assert_eq!(report.baseline.sleep_score_mean, None);
        // (400 + .. + 409) / 10
        assert_eq!(report.baseline.apple_sleep_minutes_mean, Some(404.5));
        assert_eq!(report.baseline.days_with_health, 10);

        let top = &report.analogs[0];
        assert_eq!(top.load.week_start, d(2026, 5, 18));
        assert_eq!(top.during.readiness_mean, None);
        assert_eq!(top.during.apple_sleep_minutes_mean, Some(409.0));
        assert_eq!(top.after.readiness_mean, None);
    }

    #[test]
    fn empty_target_week_is_matched_as_the_lightest_week() {
        // A target week with no context proceeds as an all-zero signature — an empty
        // planned week is legitimately "the lightest week", not an error.
        let hours = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let readiness = [60.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0];
        let fx = gradient_history(&hours, &readiness);

        let report = find_analogs(&fx.days, d(2026, 6, 29), 2).unwrap();
        assert_eq!(report.target.days_with_context, 0);
        assert!(report.target.features.is_empty());
        assert_eq!(
            report.analogs[0].load.week_start,
            d(2026, 3, 16),
            "the 10h week is nearest an all-zero target"
        );
        assert_eq!(report.analogs[1].load.week_start, d(2026, 3, 23));
    }

    #[test]
    fn upcoming_load_places_days_in_weeks_and_shows_gap_weeks() {
        let fx = Fixture::default()
            .cal(d(2026, 6, 30), 2.0, 1, 0)
            .cal(d(2026, 7, 3), 1.5, 2, 1)
            .toggl(d(2026, 7, 14), 6.0);

        let loads = upcoming_load(&fx.days, d(2026, 7, 1), 3);
        assert_eq!(loads.len(), 3);

        assert_eq!(loads[0].week_start, d(2026, 6, 29));
        assert_eq!(loads[0].features[FEATURE_MEETING_HOURS], 3.5);
        assert_eq!(loads[0].features[FEATURE_EVENT_COUNT], 3.0);
        assert_eq!(loads[0].features[FEATURE_EVENING_EVENTS], 1.0);
        assert!(
            !loads[0].features.contains_key(FEATURE_TRACKED_HOURS),
            "no toggl day in the week → the toggl feature must be absent, not 0.0"
        );
        assert_eq!(loads[0].days_with_context, 2);

        // The gap week is still listed, visibly empty.
        assert_eq!(loads[1].week_start, d(2026, 7, 6));
        assert!(loads[1].features.is_empty());
        assert_eq!(loads[1].days_with_context, 0);

        // Toggl-only week: only the toggl feature appears.
        assert_eq!(loads[2].week_start, d(2026, 7, 13));
        assert_eq!(loads[2].features.len(), 1);
        assert_eq!(loads[2].features[FEATURE_TRACKED_HOURS], 6.0);
        assert_eq!(loads[2].days_with_context, 1);
    }

    /// Eight candidate weeks (Mondays 2026-04-06 .. 2026-05-25): four light (10
    /// meeting hours) then four heavy (20), readiness 75 on each Monday. Current week
    /// (2026-06-22) has 25 meeting hours; recent readiness 70 (6/20) and 60 (6/24).
    /// Hand stats: meeting-hours mean 15, population std 5.
    fn capacity_fixture() -> Fixture {
        let first = d(2026, 4, 6);
        let mut fx = Fixture::default();
        for i in 0..8u64 {
            let monday = first + Days::new(7 * i);
            let hours = if i < 4 { 10.0 } else { 20.0 };
            fx = fx.cal(monday, hours, 0, 0).readiness(monday, 75.0);
        }
        fx.cal(d(2026, 6, 22), 25.0, 0, 0)
            .readiness(d(2026, 6, 20), 70.0)
            .readiness(d(2026, 6, 24), 60.0)
    }

    fn component<'a>(report: &'a CapacityReport, name: &str) -> &'a CapacityComponent {
        report
            .components
            .iter()
            .find(|c| c.name == name)
            .unwrap_or_else(|| panic!("component {name} missing from {:?}", report.components))
    }

    #[test]
    fn capacity_matches_hand_computation() {
        let fx = capacity_fixture();
        let report = capacity(&fx.days, d(2026, 6, 25)).unwrap();

        assert_eq!(report.as_of, d(2026, 6, 25));
        assert_eq!(report.week.week_start, d(2026, 6, 22));
        assert_eq!(report.week.features[FEATURE_MEETING_HOURS], 25.0);

        // recovery_debt: recent 7d readiness (70+60)/2 = 65, baseline (8×75)/8 = 75 →
        // clamp(10, 0, 25) × 1.2 = 12.
        assert!((component(&report, "recovery_debt").points - 12.0).abs() < 1e-9);
        // week_load: z(meeting) = (25−15)/5 = 2; event/evening features have std 0 and
        // contribute 0 → aggregate z = 2/3 → × 12 = 8.
        assert!((component(&report, "week_load").points - 8.0).abs() < 1e-9);
        // analog_risk: top-3 analogs are the 20h weeks (newest first); their
        // after-window readiness equals the 75 baseline (or is empty → skipped) → 0.
        assert!((component(&report, "analog_risk").points).abs() < 1e-9);

        // 100 − (12 + 8 + 0) = 80.
        assert_eq!(report.capacity_pct, 80);
        assert_eq!(report.band, CapacityBand::Comfortable);

        assert_eq!(report.analogs.len(), DEFAULT_ANALOG_COUNT);
        assert_eq!(report.analogs[0].load.week_start, d(2026, 5, 25));
    }

    #[test]
    fn capacity_is_deterministic() {
        let fx = capacity_fixture();
        let a = serde_json::to_value(capacity(&fx.days, d(2026, 6, 25)).unwrap()).unwrap();
        let b = serde_json::to_value(capacity(&fx.days, d(2026, 6, 25)).unwrap()).unwrap();
        assert_eq!(a, b, "identical input must produce an identical report");
    }

    #[test]
    fn capacity_refuses_thin_history() {
        let mut fx = capacity_fixture();
        // Drop one qualifying week entirely → 7 candidates.
        fx.days.remove(&d(2026, 4, 6));
        let err = capacity(&fx.days, d(2026, 6, 25)).unwrap_err();
        match err {
            HealthError::InsufficientHistory { needed, have } => {
                assert_eq!(needed, MIN_HISTORY_WEEKS);
                assert_eq!(have, 7);
            }
            other => panic!("expected InsufficientHistory, got: {other}"),
        }
    }

    #[test]
    fn capacity_components_are_transparent() {
        let fx = capacity_fixture();
        let report = capacity(&fx.days, d(2026, 6, 25)).unwrap();

        for name in ["recovery_debt", "week_load", "analog_risk"] {
            assert_eq!(
                report.components.iter().filter(|c| c.name == name).count(),
                1,
                "component {name} must appear exactly once — even at 0 points"
            );
        }
        assert_eq!(report.components.len(), 3);
        for c in &report.components {
            assert!(c.points >= 0.0, "{}: points must be ≥ 0", c.name);
            assert!(
                !c.detail.is_empty(),
                "{}: detail must explain itself",
                c.name
            );
        }
        let sum: f64 = report.components.iter().map(|c| c.points).sum();
        let explained = 100.0 - f64::from(report.capacity_pct);
        assert!(
            (sum - explained).abs() <= 0.5 + 1e-9,
            "every point off 100 must be attributed: Σ points = {sum}, 100 − pct = {explained}"
        );
    }

    #[test]
    fn recovery_debt_falls_back_to_apple_sleep() {
        // Same shape as capacity_fixture but Apple-only health: baseline sleep 480
        // min/night, recent 420 → 60 min of debt / 15 = 4 pre-scale points × 1.2 = 4.8.
        let first = d(2026, 4, 6);
        let mut fx = Fixture::default();
        for i in 0..8u64 {
            let monday = first + Days::new(7 * i);
            let hours = if i < 4 { 10.0 } else { 20.0 };
            fx = fx.cal(monday, hours, 0, 0).apple_sleep(monday, 480.0);
        }
        let fx = fx
            .cal(d(2026, 6, 22), 25.0, 0, 0)
            .apple_sleep(d(2026, 6, 24), 420.0);

        let report = capacity(&fx.days, d(2026, 6, 25)).unwrap();
        assert!((component(&report, "recovery_debt").points - 4.8).abs() < 1e-9);
        assert!((component(&report, "week_load").points - 8.0).abs() < 1e-9);
        assert!((component(&report, "analog_risk").points).abs() < 1e-9);
        // round(100 − 12.8) = 87.
        assert_eq!(report.capacity_pct, 87);
        assert_eq!(report.band, CapacityBand::Comfortable);
    }

    #[test]
    fn recovery_debt_is_zero_with_explanation_when_no_recent_data() {
        let mut fx = capacity_fixture();
        // Remove the two recent readiness days: nothing in the last 7 days from either
        // provider → 0 points, explained.
        fx.days.remove(&d(2026, 6, 20));
        fx.days.remove(&d(2026, 6, 24));

        let report = capacity(&fx.days, d(2026, 6, 25)).unwrap();
        let recovery = component(&report, "recovery_debt");
        assert_eq!(recovery.points, 0.0);
        assert_eq!(recovery.detail, "no recent recovery data");
        // Only week_load's 8 points remain deducted.
        assert_eq!(report.capacity_pct, 92);
    }
}
