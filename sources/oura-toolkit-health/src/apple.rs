//! Apple Health export importer: `export.zip` (or a bare `export.xml`) → per-day
//! [`AppleDay`] aggregates.
//!
//! There is no Apple cloud API — the export file IS the transport (feasibility doc §2).
//! Exports from long-time watch wearers reach multiple GB, so parsing is STREAMING
//! (quick-xml reader, never a DOM) and raw samples are never retained: only the per-day
//! aggregates leave this module.
//!
//! Attribution rules (documented in docs/cli-contract.md → import):
//! - Sleep samples attribute to the day the sample ENDS (the wake day, matching Oura).
//! - When several `sourceName`s recorded sleep for the same night (iPhone + Watch),
//!   only the source with the most total asleep time that night counts — summing both
//!   would double-count the night. `InBed` samples are ignored entirely.
//! - Quantity samples, workouts, and mindful sessions attribute to the day they START;
//!   daily means (heart rate, HRV, SpO2, temperature) weight each sample equally.
//! - Days are attributed in the LOCAL time each sample carries (`startDate`/`endDate`
//!   embed the recording offset) — the machine's timezone never enters the picture.
//! - Unknown record types are skipped silently; records with unparseable dates or
//!   values are skipped and counted, and a file that yields nothing BUT skips (or is
//!   not a `<HealthData>` document at all) is a parse error rather than an empty map.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::Path;

use chrono::{DateTime, NaiveDate};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::error::HealthError;
use crate::model::AppleDay;

/// The `what` every parse error from this importer carries.
const WHAT: &str = "Apple Health export";

/// The datetime format Apple uses throughout the export: local wall-clock time plus the
/// recording offset, e.g. `2026-06-30 08:00:00 -0500`.
const DATE_FMT: &str = "%Y-%m-%d %H:%M:%S %z";

fn parse_err(detail: impl Into<String>) -> HealthError {
    HealthError::Parse {
        what: WHAT,
        detail: detail.into(),
    }
}

/// Import an Apple Health export: `.zip` (containing `apple_health_export/export.xml`)
/// or an already-extracted `export.xml`.
pub fn import_apple_export(path: &Path) -> Result<BTreeMap<NaiveDate, AppleDay>, HealthError> {
    let file = File::open(path)?;
    match zip::ZipArchive::new(file) {
        Ok(mut archive) => {
            let entry = find_export_entry(&archive)?;
            let entry_reader = archive
                .by_name(&entry)
                .map_err(|e| parse_err(format!("could not read zip entry {entry}: {e}")))?;
            parse_export_xml(BufReader::new(entry_reader))
        }
        Err(zip::result::ZipError::Io(e)) => Err(HealthError::Io(e)),
        // Not a zip archive — treat the file as a bare export.xml. If it is not an
        // Apple Health XML document either, the parser reports that below.
        Err(_) => parse_export_xml(BufReader::new(File::open(path)?)),
    }
}

/// True for a zip entry that can be the export XML: named `export.xml`
/// (case-insensitively — some regions ship `Export.xml`) and at the top level or one
/// directory deep (Apple nests it as `apple_health_export/export.xml`).
fn is_export_entry(name: &str) -> bool {
    let parts: Vec<&str> = name.split('/').filter(|p| !p.is_empty()).collect();
    parts.len() <= 2
        && parts
            .last()
            .is_some_and(|f| f.eq_ignore_ascii_case("export.xml"))
}

/// Pick the export XML entry from the archive. Falls back to a lone shallow `*.xml`
/// (regional exports occasionally rename the file) but never guesses between several.
fn find_export_entry<R: Read + Seek>(archive: &zip::ZipArchive<R>) -> Result<String, HealthError> {
    if let Some(name) = archive.file_names().find(|n| is_export_entry(n)) {
        return Ok(name.to_owned());
    }
    let shallow_xml: Vec<&str> = archive
        .file_names()
        .filter(|n| {
            let parts: Vec<&str> = n.split('/').filter(|p| !p.is_empty()).collect();
            parts.len() <= 2
                && parts
                    .last()
                    .is_some_and(|f| f.to_ascii_lowercase().ends_with(".xml"))
        })
        .collect();
    match shallow_xml.as_slice() {
        [only] => Ok((*only).to_owned()),
        [] => Err(parse_err(
            "the zip contains no export.xml (expected Apple's export.zip with \
             apple_health_export/export.xml, or pass the export.xml directly)",
        )),
        many => Err(parse_err(format!(
            "the zip contains no export.xml and {} other .xml entries — pass the \
             export XML directly",
            many.len()
        ))),
    }
}

/// Stream the export XML, folding every recognized `<Record>`/`<Workout>` into per-day
/// accumulators. Memory stays O(one record): the reader buffer is reused and no sample
/// outlives its dispatch.
fn parse_export_xml<R: BufRead>(input: R) -> Result<BTreeMap<NaiveDate, AppleDay>, HealthError> {
    let mut reader = Reader::from_reader(input);
    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();
    let mut acc = Accumulator::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(parse_err(format!("malformed XML: {e}"))),
            Ok(Event::Eof) => break,
            // A Start form means the element has children (MetadataEntry,
            // HeartRateVariabilityMetadataList, workout events) — dispatch on the
            // attributes, then skip the whole subtree.
            Ok(Event::Start(e)) => {
                let handled = match e.local_name().as_ref() {
                    b"HealthData" => {
                        acc.saw_health_data = true;
                        false
                    }
                    b"Record" => {
                        acc.record(&e);
                        true
                    }
                    b"Workout" => {
                        acc.workout(&e);
                        true
                    }
                    _ => false,
                };
                if handled {
                    let end = e.to_end().into_owned();
                    reader
                        .read_to_end_into(end.name(), &mut skip_buf)
                        .map_err(|e| parse_err(format!("malformed XML: {e}")))?;
                }
            }
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"HealthData" => acc.saw_health_data = true,
                b"Record" => acc.record(&e),
                b"Workout" => acc.workout(&e),
                _ => {}
            },
            Ok(_) => {}
        }
        buf.clear();
    }
    acc.finish()
}

/// The attributes this importer reads off a `<Record>`/`<Workout>` element.
#[derive(Default)]
struct RawAttrs {
    r#type: Option<String>,
    source: Option<String>,
    value: Option<String>,
    start: Option<String>,
    end: Option<String>,
    duration: Option<String>,
    duration_unit: Option<String>,
}

/// Extract the attributes we care about; `None` means the element's attribute syntax
/// itself is broken (counted as a skipped record by callers).
fn attrs(e: &BytesStart) -> Option<RawAttrs> {
    let mut out = RawAttrs::default();
    for a in e.attributes() {
        let a = a.ok()?;
        let val = a
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .ok()?;
        match a.key.as_ref() {
            b"type" => out.r#type = Some(val.into_owned()),
            b"sourceName" => out.source = Some(val.into_owned()),
            b"value" => out.value = Some(val.into_owned()),
            b"startDate" => out.start = Some(val.into_owned()),
            b"endDate" => out.end = Some(val.into_owned()),
            b"duration" => out.duration = Some(val.into_owned()),
            b"durationUnit" => out.duration_unit = Some(val.into_owned()),
            _ => {}
        }
    }
    Some(out)
}

/// The LOCAL calendar day the timestamp string encodes — parse WITH the embedded
/// offset and take the wall-clock date as written, never converting to the machine tz.
fn local_date(s: &str) -> Option<NaiveDate> {
    DateTime::parse_from_str(s, DATE_FMT)
        .ok()
        .map(|dt| dt.date_naive())
}

/// end − start in minutes; `None` when either timestamp is unparseable or the span is
/// negative.
fn duration_minutes(start: &str, end: &str) -> Option<f64> {
    let s = DateTime::parse_from_str(start, DATE_FMT).ok()?;
    let e = DateTime::parse_from_str(end, DATE_FMT).ok()?;
    let mins = (e - s).num_seconds() as f64 / 60.0;
    (mins >= 0.0).then_some(mins)
}

/// A sample-equal-weight daily mean.
#[derive(Default)]
struct MeanAcc {
    sum: f64,
    n: u32,
}

impl MeanAcc {
    fn push(&mut self, v: f64) {
        self.sum += v;
        self.n += 1;
    }
    fn mean(&self) -> Option<f64> {
        (self.n > 0).then(|| self.sum / f64::from(self.n))
    }
}

/// Everything one day accumulates except sleep (which needs per-source resolution
/// first — see [`SleepAcc`]). `Option` sums start `None` so a field is `Some(0.0)` only
/// when a record actually measured zero.
#[derive(Default)]
struct DayAcc {
    steps: Option<f64>,
    active_energy_kcal: Option<f64>,
    exercise_minutes: Option<f64>,
    stand_hours: Option<f64>,
    mindful_minutes: Option<f64>,
    resting_hr: MeanAcc,
    hrv_sdnn: MeanAcc,
    resp_rate: MeanAcc,
    spo2: MeanAcc,
    wrist_temp: MeanAcc,
    vo2_max: MeanAcc,
    workout_count: u32,
    workout_minutes: Option<f64>,
}

/// One source's sleep totals for one wake day. Stage fields stay `None` unless that
/// stage was actually sampled (an iPhone-only night has no stages, and `None` ≠
/// measured-zero).
#[derive(Default)]
struct SleepAcc {
    asleep: Option<f64>,
    deep: Option<f64>,
    rem: Option<f64>,
    core: Option<f64>,
    awake: Option<f64>,
}

fn add(slot: &mut Option<f64>, v: f64) {
    *slot = Some(slot.unwrap_or(0.0) + v);
}

#[derive(Default)]
struct Accumulator {
    days: BTreeMap<NaiveDate, DayAcc>,
    /// wake day → sourceName → that source's sleep totals.
    sleep: BTreeMap<NaiveDate, BTreeMap<String, SleepAcc>>,
    saw_health_data: bool,
    recognized: u64,
    skipped: u64,
}

impl Accumulator {
    fn record(&mut self, e: &BytesStart) {
        let Some(a) = attrs(e) else {
            self.skipped += 1;
            return;
        };
        let Some(ty) = a.r#type.as_deref() else {
            // A <Record> without a type cannot be classified — that's a broken record,
            // not an unknown-but-valid type.
            self.skipped += 1;
            return;
        };
        match ty {
            "HKQuantityTypeIdentifierStepCount" => self.sum_value(&a, |d| &mut d.steps),
            "HKQuantityTypeIdentifierActiveEnergyBurned" => {
                self.sum_value(&a, |d| &mut d.active_energy_kcal)
            }
            "HKQuantityTypeIdentifierAppleExerciseTime" => {
                self.sum_value(&a, |d| &mut d.exercise_minutes)
            }
            // The stand hour is a category type in real exports; the quantity spelling
            // is accepted for robustness.
            "HKCategoryTypeIdentifierAppleStandHour" | "HKQuantityTypeIdentifierAppleStandHour" => {
                self.stand_hour(&a)
            }
            "HKCategoryTypeIdentifierMindfulSession" => self.mindful(&a),
            "HKQuantityTypeIdentifierRestingHeartRate" => {
                self.mean_value(&a, |d| &mut d.resting_hr)
            }
            "HKQuantityTypeIdentifierHeartRateVariabilitySDNN" => {
                self.mean_value(&a, |d| &mut d.hrv_sdnn)
            }
            "HKQuantityTypeIdentifierRespiratoryRate" => self.mean_value(&a, |d| &mut d.resp_rate),
            "HKQuantityTypeIdentifierOxygenSaturation" => self.spo2(&a),
            "HKQuantityTypeIdentifierAppleSleepingWristTemperature" => {
                self.mean_value(&a, |d| &mut d.wrist_temp)
            }
            "HKQuantityTypeIdentifierVO2Max" => self.mean_value(&a, |d| &mut d.vo2_max),
            "HKCategoryTypeIdentifierSleepAnalysis" => self.sleep(&a),
            _ => {} // unknown type: silently skipped, by design
        }
    }

    /// start-day + finite numeric `value`, or a counted skip.
    fn parsed_value(&mut self, a: &RawAttrs) -> Option<(NaiveDate, f64)> {
        let parsed = a
            .start
            .as_deref()
            .and_then(local_date)
            .zip(a.value.as_deref().and_then(|v| v.parse::<f64>().ok()))
            .filter(|(_, v)| v.is_finite());
        if parsed.is_none() {
            self.skipped += 1;
        }
        parsed
    }

    fn sum_value(&mut self, a: &RawAttrs, field: impl Fn(&mut DayAcc) -> &mut Option<f64>) {
        if let Some((day, v)) = self.parsed_value(a) {
            add(field(self.days.entry(day).or_default()), v);
            self.recognized += 1;
        }
    }

    fn mean_value(&mut self, a: &RawAttrs, field: impl Fn(&mut DayAcc) -> &mut MeanAcc) {
        if let Some((day, v)) = self.parsed_value(a) {
            field(self.days.entry(day).or_default()).push(v);
            self.recognized += 1;
        }
    }

    /// Exports store SpO2 as a fraction (unit `%`, value `0.97`): values ≤ 1 scale to
    /// percent; values > 1 are already percent.
    fn spo2(&mut self, a: &RawAttrs) {
        if let Some((day, v)) = self.parsed_value(a) {
            let pct = if v <= 1.0 { v * 100.0 } else { v };
            self.days.entry(day).or_default().spo2.push(pct);
            self.recognized += 1;
        }
    }

    /// Any parseable stand-hour record marks the day measured; only
    /// `…AppleStandHourStood` increments the count (so an all-idle day is a true zero).
    fn stand_hour(&mut self, a: &RawAttrs) {
        let (Some(day), Some(value)) = (a.start.as_deref().and_then(local_date), &a.value) else {
            self.skipped += 1;
            return;
        };
        let inc = if value == "HKCategoryValueAppleStandHourStood" {
            1.0
        } else {
            0.0
        };
        add(&mut self.days.entry(day).or_default().stand_hours, inc);
        self.recognized += 1;
    }

    fn mindful(&mut self, a: &RawAttrs) {
        let parsed = a.start.as_deref().and_then(local_date).zip(
            a.start
                .as_deref()
                .zip(a.end.as_deref())
                .and_then(|(s, e)| duration_minutes(s, e)),
        );
        let Some((day, mins)) = parsed else {
            self.skipped += 1;
            return;
        };
        add(&mut self.days.entry(day).or_default().mindful_minutes, mins);
        self.recognized += 1;
    }

    fn sleep(&mut self, a: &RawAttrs) {
        let value = a.value.as_deref().unwrap_or("");
        // InBed is not sleep; ignore it entirely (neither recognized nor a skip).
        // Unrecognized stage values are treated the same so a future HK enum value
        // cannot turn a valid export into a parse error.
        let stage = match value {
            "HKCategoryValueSleepAnalysisAsleepDeep" => Stage::Deep,
            "HKCategoryValueSleepAnalysisAsleepREM" => Stage::Rem,
            "HKCategoryValueSleepAnalysisAsleepCore" => Stage::Core,
            "HKCategoryValueSleepAnalysisAsleepUnspecified" => Stage::Unspecified,
            "HKCategoryValueSleepAnalysisAwake" => Stage::Awake,
            _ => return,
        };
        // Sleep attributes to the day the sample ENDS: the wake day.
        let parsed = a.end.as_deref().and_then(local_date).zip(
            a.start
                .as_deref()
                .zip(a.end.as_deref())
                .and_then(|(s, e)| duration_minutes(s, e)),
        );
        let Some((wake_day, mins)) = parsed else {
            self.skipped += 1;
            return;
        };
        let source = a.source.clone().unwrap_or_default();
        let acc = self
            .sleep
            .entry(wake_day)
            .or_default()
            .entry(source)
            .or_default();
        match stage {
            Stage::Deep => {
                add(&mut acc.asleep, mins);
                add(&mut acc.deep, mins);
            }
            Stage::Rem => {
                add(&mut acc.asleep, mins);
                add(&mut acc.rem, mins);
            }
            Stage::Core => {
                add(&mut acc.asleep, mins);
                add(&mut acc.core, mins);
            }
            Stage::Unspecified => add(&mut acc.asleep, mins),
            Stage::Awake => add(&mut acc.awake, mins),
        }
        self.recognized += 1;
    }

    fn workout(&mut self, e: &BytesStart) {
        let Some(a) = attrs(e) else {
            self.skipped += 1;
            return;
        };
        let Some(day) = a.start.as_deref().and_then(local_date) else {
            self.skipped += 1;
            return;
        };
        // Honor `duration` when its unit is minutes (or convertible); otherwise fall
        // back to end − start.
        let from_attr = a
            .duration
            .as_deref()
            .and_then(|d| d.parse::<f64>().ok())
            .filter(|v| v.is_finite())
            .and_then(|v| match a.duration_unit.as_deref() {
                Some("min") | None => Some(v),
                Some("sec") => Some(v / 60.0),
                Some("hr") => Some(v * 60.0),
                Some(_) => None,
            });
        let mins = from_attr.or_else(|| {
            a.start
                .as_deref()
                .zip(a.end.as_deref())
                .and_then(|(s, e)| duration_minutes(s, e))
        });
        let Some(mins) = mins else {
            self.skipped += 1;
            return;
        };
        let d = self.days.entry(day).or_default();
        d.workout_count += 1;
        add(&mut d.workout_minutes, mins);
        self.recognized += 1;
    }

    fn finish(self) -> Result<BTreeMap<NaiveDate, AppleDay>, HealthError> {
        if self.recognized == 0 {
            if !self.saw_health_data {
                return Err(parse_err(
                    "not an Apple Health export (<HealthData> root not found) — pass \
                     Apple's export.zip or the export.xml inside it",
                ));
            }
            if self.skipped > 0 {
                return Err(parse_err(format!(
                    "no recognizable records — {} record(s) skipped as unparseable",
                    self.skipped
                )));
            }
        }

        let mut out: BTreeMap<NaiveDate, AppleDay> = BTreeMap::new();
        for (day, d) in self.days {
            out.insert(
                day,
                AppleDay {
                    resting_hr_bpm: d.resting_hr.mean(),
                    hrv_sdnn_ms: d.hrv_sdnn.mean(),
                    respiratory_rate: d.resp_rate.mean(),
                    spo2_pct: d.spo2.mean(),
                    wrist_temp_c: d.wrist_temp.mean(),
                    vo2_max: d.vo2_max.mean(),
                    steps: d.steps,
                    active_energy_kcal: d.active_energy_kcal,
                    exercise_minutes: d.exercise_minutes,
                    stand_hours: d.stand_hours,
                    mindful_minutes: d.mindful_minutes,
                    workout_count: (d.workout_count > 0).then_some(d.workout_count),
                    workout_minutes: d.workout_minutes,
                    ..Default::default()
                },
            );
        }
        for (day, sources) in self.sleep {
            // Per wake day, only the source with the most total asleep time counts.
            let winner = sources
                .into_iter()
                .max_by(|(_, x), (_, y)| {
                    x.asleep
                        .unwrap_or(0.0)
                        .partial_cmp(&y.asleep.unwrap_or(0.0))
                        .unwrap_or(Ordering::Equal)
                })
                .map(|(_, s)| s)
                .expect("sleep day entries are created only when a sample lands");
            let entry = out.entry(day).or_default();
            entry.sleep_minutes = winner.asleep;
            entry.deep_minutes = winner.deep;
            entry.rem_minutes = winner.rem;
            entry.core_minutes = winner.core;
            entry.awake_minutes = winner.awake;
        }
        Ok(out)
    }
}

/// The sleep-analysis stages this importer aggregates (InBed is deliberately absent).
enum Stage {
    Deep,
    Rem,
    Core,
    Unspecified,
    Awake,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_entry_matcher_accepts_shallow_export_xml_case_insensitively() {
        assert!(is_export_entry("export.xml"));
        assert!(is_export_entry("Export.xml"));
        assert!(is_export_entry("apple_health_export/export.xml"));
        assert!(is_export_entry("apple_health_export/EXPORT.XML"));
        // Two directories deep or differently named entries are not the export.
        assert!(!is_export_entry("a/b/export.xml"));
        assert!(!is_export_entry("apple_health_export/export_cda.xml"));
        assert!(!is_export_entry("export.json"));
    }

    #[test]
    fn local_date_takes_the_wall_clock_day_of_the_embedded_offset() {
        // 23:30 -0500 is 04:30 UTC the NEXT day — the local day must win.
        assert_eq!(
            local_date("2026-06-30 23:30:00 -0500"),
            NaiveDate::from_ymd_opt(2026, 6, 30)
        );
        assert_eq!(local_date("2026-06-30T23:30:00Z"), None);
        assert_eq!(local_date("not a date"), None);
    }

    #[test]
    fn duration_minutes_rejects_negative_spans() {
        assert_eq!(
            duration_minutes("2026-06-30 08:00:00 -0500", "2026-06-30 08:10:00 -0500"),
            Some(10.0)
        );
        assert_eq!(
            duration_minutes("2026-06-30 08:10:00 -0500", "2026-06-30 08:00:00 -0500"),
            None
        );
    }
}
