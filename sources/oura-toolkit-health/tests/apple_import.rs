//! Integration tests for the Apple Health export importer. All fixtures are built
//! in-test from string XML (and the zip crate's writer) into tempdirs — no binary
//! fixtures, no network, no shared state.

use std::io::Write;
use std::path::PathBuf;

use chrono::NaiveDate;
use oura_toolkit_health::apple::import_apple_export;
use oura_toolkit_health::{AppleDay, HealthError};
use tempfile::TempDir;

fn day(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

/// `<Record .../>` with the attribute set real exports use.
fn record(ty: &str, source: &str, start: &str, end: &str, value: &str) -> String {
    format!(
        r#"<Record type="{ty}" sourceName="{source}" sourceVersion="1" unit="count" creationDate="{end}" startDate="{start}" endDate="{end}" value="{value}"/>"#
    )
}

fn export(body: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE HealthData>
<HealthData locale="en_US">
 <ExportDate value="2026-07-02 09:00:00 -0500"/>
 <Me HKCharacteristicTypeIdentifierBiologicalSex="HKBiologicalSexNotSet"/>
{body}
</HealthData>"#
    )
}

fn write_xml(dir: &TempDir, name: &str, xml: &str) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, xml).unwrap();
    path
}

fn write_zip(dir: &TempDir, zip_name: &str, entry: &str, xml: &str) -> PathBuf {
    let path = dir.path().join(zip_name);
    let file = std::fs::File::create(&path).unwrap();
    let mut writer = zip::ZipWriter::new(file);
    writer
        .start_file(entry, zip::write::SimpleFileOptions::default())
        .unwrap();
    writer.write_all(xml.as_bytes()).unwrap();
    writer.finish().unwrap();
    path
}

fn assert_approx(got: Option<f64>, want: f64, field: &str) {
    let got = got.unwrap_or_else(|| panic!("{field} should be Some({want})"));
    assert!(
        (got - want).abs() < 1e-9,
        "{field}: expected {want}, got {got}"
    );
}

/// The realistic multi-day fixture: every mapped type, hand-computed expectations.
fn multi_day_xml() -> String {
    let body = [
        // ---- 2026-06-29: everything ----
        // steps: 612 + 388 = 1000
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "2026-06-29 08:00:00 -0500",
            "2026-06-29 08:10:00 -0500",
            "612",
        ),
        record(
            "HKQuantityTypeIdentifierStepCount",
            "Watch",
            "2026-06-29 12:00:00 -0500",
            "2026-06-29 12:10:00 -0500",
            "388",
        ),
        // active energy: 250.5 + 249.5 = 500
        record(
            "HKQuantityTypeIdentifierActiveEnergyBurned",
            "Watch",
            "2026-06-29 09:00:00 -0500",
            "2026-06-29 10:00:00 -0500",
            "250.5",
        ),
        record(
            "HKQuantityTypeIdentifierActiveEnergyBurned",
            "Watch",
            "2026-06-29 15:00:00 -0500",
            "2026-06-29 16:00:00 -0500",
            "249.5",
        ),
        // exercise minutes: 12 + 18 = 30
        record(
            "HKQuantityTypeIdentifierAppleExerciseTime",
            "Watch",
            "2026-06-29 09:00:00 -0500",
            "2026-06-29 09:12:00 -0500",
            "12",
        ),
        record(
            "HKQuantityTypeIdentifierAppleExerciseTime",
            "Watch",
            "2026-06-29 17:00:00 -0500",
            "2026-06-29 17:18:00 -0500",
            "18",
        ),
        // stand hours: Stood, Stood, Idle -> 2
        record(
            "HKCategoryTypeIdentifierAppleStandHour",
            "Watch",
            "2026-06-29 09:00:00 -0500",
            "2026-06-29 10:00:00 -0500",
            "HKCategoryValueAppleStandHourStood",
        ),
        record(
            "HKCategoryTypeIdentifierAppleStandHour",
            "Watch",
            "2026-06-29 10:00:00 -0500",
            "2026-06-29 11:00:00 -0500",
            "HKCategoryValueAppleStandHourStood",
        ),
        record(
            "HKCategoryTypeIdentifierAppleStandHour",
            "Watch",
            "2026-06-29 11:00:00 -0500",
            "2026-06-29 12:00:00 -0500",
            "HKCategoryValueAppleStandHourIdle",
        ),
        // mindful: 10:00-10:10 -> 10 min
        record(
            "HKCategoryTypeIdentifierMindfulSession",
            "iPhone",
            "2026-06-29 10:00:00 -0500",
            "2026-06-29 10:10:00 -0500",
            "HKCategoryValueNotApplicable",
        ),
        // resting HR mean: (52 + 56) / 2 = 54
        record(
            "HKQuantityTypeIdentifierRestingHeartRate",
            "Watch",
            "2026-06-29 07:00:00 -0500",
            "2026-06-29 07:00:00 -0500",
            "52",
        ),
        record(
            "HKQuantityTypeIdentifierRestingHeartRate",
            "Watch",
            "2026-06-29 20:00:00 -0500",
            "2026-06-29 20:00:00 -0500",
            "56",
        ),
        // HRV SDNN mean: (45 + 55) / 2 = 50; the second one carries a nested
        // metadata list to prove Start-form records with children parse.
        record(
            "HKQuantityTypeIdentifierHeartRateVariabilitySDNN",
            "Watch",
            "2026-06-29 06:00:00 -0500",
            "2026-06-29 06:01:00 -0500",
            "45",
        ),
        r#"<Record type="HKQuantityTypeIdentifierHeartRateVariabilitySDNN" sourceName="Watch" unit="ms" startDate="2026-06-29 22:00:00 -0500" endDate="2026-06-29 22:01:00 -0500" value="55">
  <HeartRateVariabilityMetadataList>
    <InstantaneousBeatsPerMinute bpm="61" time="10:04:06.61 PM"/>
    <InstantaneousBeatsPerMinute bpm="63" time="10:04:07.59 PM"/>
  </HeartRateVariabilityMetadataList>
</Record>"#
            .to_string(),
        // respiratory rate mean: (14 + 16) / 2 = 15
        record(
            "HKQuantityTypeIdentifierRespiratoryRate",
            "Watch",
            "2026-06-29 03:00:00 -0500",
            "2026-06-29 03:05:00 -0500",
            "14",
        ),
        record(
            "HKQuantityTypeIdentifierRespiratoryRate",
            "Watch",
            "2026-06-29 04:00:00 -0500",
            "2026-06-29 04:05:00 -0500",
            "16",
        ),
        // SpO2 (fractions): (96 + 98) / 2 = 97
        record(
            "HKQuantityTypeIdentifierOxygenSaturation",
            "Watch",
            "2026-06-29 02:00:00 -0500",
            "2026-06-29 02:00:00 -0500",
            "0.96",
        ),
        record(
            "HKQuantityTypeIdentifierOxygenSaturation",
            "Watch",
            "2026-06-29 03:00:00 -0500",
            "2026-06-29 03:00:00 -0500",
            "0.98",
        ),
        // wrist temp mean: (36.4 + 36.6) / 2 = 36.5
        record(
            "HKQuantityTypeIdentifierAppleSleepingWristTemperature",
            "Watch",
            "2026-06-29 01:00:00 -0500",
            "2026-06-29 05:00:00 -0500",
            "36.4",
        ),
        record(
            "HKQuantityTypeIdentifierAppleSleepingWristTemperature",
            "Watch",
            "2026-06-29 05:00:00 -0500",
            "2026-06-29 06:00:00 -0500",
            "36.6",
        ),
        // VO2Max: single sample 41.2
        record(
            "HKQuantityTypeIdentifierVO2Max",
            "Watch",
            "2026-06-29 09:30:00 -0500",
            "2026-06-29 09:30:00 -0500",
            "41.2",
        ),
        // one workout with duration attr in minutes
        r#"<Workout workoutActivityType="HKWorkoutActivityTypeRunning" duration="30" durationUnit="min" sourceName="Watch" startDate="2026-06-29 09:00:00 -0500" endDate="2026-06-29 09:30:00 -0500">
  <MetadataEntry key="HKIndoorWorkout" value="0"/>
  <WorkoutEvent type="HKWorkoutEventTypeSegment" date="2026-06-29 09:00:00 -0500"/>
</Workout>"#
            .to_string(),
        // sleep ending the morning of 06-29 (Watch, staged):
        // deep 60 + rem 90 + core 240 + unspecified 10 = asleep 400; awake 20;
        // one InBed sample that must be ignored entirely.
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Watch",
            "2026-06-28 23:00:00 -0500",
            "2026-06-29 00:00:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepDeep",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Watch",
            "2026-06-29 00:00:00 -0500",
            "2026-06-29 01:30:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepREM",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Watch",
            "2026-06-29 01:30:00 -0500",
            "2026-06-29 05:30:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepCore",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Watch",
            "2026-06-29 05:30:00 -0500",
            "2026-06-29 05:40:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepUnspecified",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Watch",
            "2026-06-29 05:40:00 -0500",
            "2026-06-29 06:00:00 -0500",
            "HKCategoryValueSleepAnalysisAwake",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "iPhone",
            "2026-06-28 22:50:00 -0500",
            "2026-06-29 06:10:00 -0500",
            "HKCategoryValueSleepAnalysisInBed",
        ),
        // ---- 2026-06-30: steps + two workouts, nothing else ----
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "2026-06-30 10:00:00 -0500",
            "2026-06-30 10:10:00 -0500",
            "5000",
        ),
        // no duration attr -> computed from end - start = 45 min
        r#"<Workout workoutActivityType="HKWorkoutActivityTypeWalking" sourceName="Watch" startDate="2026-06-30 07:00:00 -0500" endDate="2026-06-30 07:45:00 -0500"/>"#.to_string(),
        r#"<Workout workoutActivityType="HKWorkoutActivityTypeYoga" duration="20" durationUnit="min" sourceName="Watch" startDate="2026-06-30 18:00:00 -0500" endDate="2026-06-30 18:20:00 -0500"/>"#.to_string(),
    ]
    .join("\n");
    export(&body)
}

#[test]
fn multi_day_export_aggregates_every_mapped_type() {
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &multi_day_xml());
    let days = import_apple_export(&path).unwrap();

    assert_eq!(
        days.keys().copied().collect::<Vec<_>>(),
        vec![day(2026, 6, 29), day(2026, 6, 30)],
        "exactly the two days with recognized records exist"
    );

    let d1 = &days[&day(2026, 6, 29)];
    assert_approx(d1.steps, 1000.0, "steps");
    assert_approx(d1.active_energy_kcal, 500.0, "active_energy_kcal");
    assert_approx(d1.exercise_minutes, 30.0, "exercise_minutes");
    assert_approx(d1.stand_hours, 2.0, "stand_hours (only Stood counts)");
    assert_approx(d1.mindful_minutes, 10.0, "mindful_minutes");
    assert_approx(d1.resting_hr_bpm, 54.0, "resting_hr_bpm");
    assert_approx(d1.hrv_sdnn_ms, 50.0, "hrv_sdnn_ms");
    assert_approx(d1.respiratory_rate, 15.0, "respiratory_rate");
    assert_approx(d1.spo2_pct, 97.0, "spo2_pct");
    assert_approx(d1.wrist_temp_c, 36.5, "wrist_temp_c");
    assert_approx(d1.vo2_max, 41.2, "vo2_max");
    assert_eq!(d1.workout_count, Some(1), "workout_count");
    assert_approx(d1.workout_minutes, 30.0, "workout_minutes");
    assert_approx(d1.sleep_minutes, 400.0, "sleep_minutes (InBed excluded)");
    assert_approx(d1.deep_minutes, 60.0, "deep_minutes");
    assert_approx(d1.rem_minutes, 90.0, "rem_minutes");
    assert_approx(d1.core_minutes, 240.0, "core_minutes");
    assert_approx(d1.awake_minutes, 20.0, "awake_minutes");

    let d2 = &days[&day(2026, 6, 30)];
    assert_approx(d2.steps, 5000.0, "day-2 steps");
    assert_eq!(d2.workout_count, Some(2), "day-2 workout_count");
    assert_approx(
        d2.workout_minutes,
        65.0,
        "day-2 workout_minutes (45 computed + 20 from duration attr)",
    );
    // Fields no record measured stay None — never a default zero.
    assert_eq!(d2.sleep_minutes, None, "day-2 sleep_minutes");
    assert_eq!(d2.resting_hr_bpm, None, "day-2 resting_hr_bpm");
    assert_eq!(d2.stand_hours, None, "day-2 stand_hours");
    assert_eq!(d2.exercise_minutes, None, "day-2 exercise_minutes");
    assert_eq!(d2.mindful_minutes, None, "day-2 mindful_minutes");
}

#[test]
fn overlapping_sleep_sources_use_the_largest_not_the_sum() {
    // The same night recorded by Watch (staged, 390 min asleep) and iPhone
    // (unspecified, 330 min). Summing would report 720 — the contract is that only
    // the largest source counts.
    let body = [
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Apple Watch",
            "2026-06-30 23:00:00 -0500",
            "2026-07-01 03:00:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepCore",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Apple Watch",
            "2026-07-01 03:00:00 -0500",
            "2026-07-01 04:00:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepDeep",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Apple Watch",
            "2026-07-01 04:00:00 -0500",
            "2026-07-01 05:30:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepREM",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "Apple Watch",
            "2026-07-01 05:30:00 -0500",
            "2026-07-01 05:45:00 -0500",
            "HKCategoryValueSleepAnalysisAwake",
        ),
        record(
            "HKCategoryTypeIdentifierSleepAnalysis",
            "iPhone",
            "2026-06-30 23:30:00 -0500",
            "2026-07-01 05:00:00 -0500",
            "HKCategoryValueSleepAnalysisAsleepUnspecified",
        ),
    ]
    .join("\n");
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();

    let night = &days[&day(2026, 7, 1)];
    assert_approx(
        night.sleep_minutes,
        390.0,
        "sleep_minutes = Watch's 240+60+90, NOT 720 (both sources summed) and NOT 330 (iPhone)",
    );
    assert_approx(night.deep_minutes, 60.0, "deep from the winning source");
    assert_approx(night.rem_minutes, 90.0, "rem from the winning source");
    assert_approx(night.core_minutes, 240.0, "core from the winning source");
    assert_approx(night.awake_minutes, 15.0, "awake from the winning source");
}

#[test]
fn sleep_crossing_midnight_attributes_to_the_wake_day() {
    let body = record(
        "HKCategoryTypeIdentifierSleepAnalysis",
        "Apple Watch",
        "2026-06-30 23:00:00 -0500",
        "2026-07-01 06:00:00 -0500",
        "HKCategoryValueSleepAnalysisAsleepCore",
    );
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();

    assert_eq!(
        days.keys().copied().collect::<Vec<_>>(),
        vec![day(2026, 7, 1)],
        "the whole sample lands on the day it ENDS (the wake day)"
    );
    assert_approx(days[&day(2026, 7, 1)].sleep_minutes, 420.0, "sleep_minutes");
}

#[test]
fn zip_and_bare_xml_inputs_produce_identical_results() {
    let xml = multi_day_xml();
    let dir = TempDir::new().unwrap();
    let bare = write_xml(&dir, "export.xml", &xml);
    let nested = write_zip(&dir, "export.zip", "apple_health_export/export.xml", &xml);
    let root_cased = write_zip(&dir, "export2.zip", "Export.xml", &xml);

    let from_bare = import_apple_export(&bare).unwrap();
    let from_nested = import_apple_export(&nested).unwrap();
    let from_root_cased = import_apple_export(&root_cased).unwrap();

    assert!(!from_bare.is_empty(), "fixture parses to a non-empty map");
    assert_eq!(
        from_nested, from_bare,
        "zip (apple_health_export/export.xml) == bare xml"
    );
    assert_eq!(
        from_root_cased, from_bare,
        "zip with root-level Export.xml (case-insensitive) == bare xml"
    );
}

#[test]
fn unknown_record_types_are_skipped_silently() {
    let body = [
        record(
            "HKQuantityTypeIdentifierBodyMass",
            "Scale",
            "2026-06-29 08:00:00 -0500",
            "2026-06-29 08:00:00 -0500",
            "80",
        ),
        record(
            "HKQuantityTypeIdentifierDietaryCaffeine",
            "App",
            "2026-06-29 09:00:00 -0500",
            "2026-06-29 09:00:00 -0500",
            "120",
        ),
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "2026-06-29 10:00:00 -0500",
            "2026-06-29 10:05:00 -0500",
            "700",
        ),
    ]
    .join("\n");
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();

    let d = &days[&day(2026, 6, 29)];
    assert_approx(d.steps, 700.0, "steps unaffected by unknown types");
    // Nothing else on the day was measured by a recognized record.
    assert_eq!((d.active_energy_kcal, d.resting_hr_bpm), (None, None));
}

#[test]
fn valid_export_with_only_unknown_types_is_an_empty_map() {
    let body = record(
        "HKQuantityTypeIdentifierBodyMass",
        "Scale",
        "2026-06-29 08:00:00 -0500",
        "2026-06-29 08:00:00 -0500",
        "80",
    );
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    assert!(
        days.is_empty(),
        "a well-formed export with no relevant records is Ok(empty), not an error"
    );
}

#[test]
fn non_export_file_is_a_parse_error_naming_the_input_kind() {
    let dir = TempDir::new().unwrap();
    let path = write_xml(
        &dir,
        "not-health.txt",
        "this is neither a zip nor health XML",
    );
    let err = import_apple_export(&path).unwrap_err();
    match err {
        HealthError::Parse { what, detail } => {
            assert_eq!(what, "Apple Health export");
            assert!(
                detail.contains("export.zip") && detail.contains("export.xml"),
                "the error tells the user what input is expected: {detail}"
            );
        }
        other => panic!("expected HealthError::Parse, got {other:?}"),
    }
}

#[test]
fn xml_that_is_not_healthdata_is_a_parse_error() {
    // Well-formed XML, but not a HealthData document and no recognizable records —
    // importing it silently would bless arbitrary XML as health history.
    let dir = TempDir::new().unwrap();
    let path = write_xml(
        &dir,
        "other.xml",
        r#"<?xml version="1.0"?><Recipes><Recipe name="soup"/></Recipes>"#,
    );
    match import_apple_export(&path).unwrap_err() {
        HealthError::Parse { what, detail } => {
            assert_eq!(what, "Apple Health export");
            assert!(
                detail.contains("HealthData"),
                "the error names the missing root: {detail}"
            );
        }
        other => panic!("expected HealthError::Parse, got {other:?}"),
    }
}

#[test]
fn missing_file_is_an_io_error() {
    let dir = TempDir::new().unwrap();
    let err = import_apple_export(&dir.path().join("nope.zip")).unwrap_err();
    assert!(
        matches!(err, HealthError::Io(_)),
        "missing file surfaces the underlying Io error, got {err:?}"
    );
}

#[test]
fn spo2_fractions_convert_to_percent_and_percents_pass_through() {
    let body = [
        record(
            "HKQuantityTypeIdentifierOxygenSaturation",
            "Watch",
            "2026-06-29 02:00:00 -0500",
            "2026-06-29 02:00:00 -0500",
            "0.97",
        ),
        // already-percent value (> 1) must NOT be scaled again
        record(
            "HKQuantityTypeIdentifierOxygenSaturation",
            "Watch",
            "2026-06-29 03:00:00 -0500",
            "2026-06-29 03:00:00 -0500",
            "99",
        ),
    ]
    .join("\n");
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    assert_approx(
        days[&day(2026, 6, 29)].spo2_pct,
        98.0,
        "mean(0.97 -> 97, 99 stays 99)",
    );
}

#[test]
fn timezone_offset_in_the_record_wins_over_utc() {
    // 2026-06-30 23:30 -0500 is 2026-07-01 04:30 UTC. The record must land on the
    // LOCAL day written in the export, June 30 — never the UTC day.
    let body = record(
        "HKQuantityTypeIdentifierStepCount",
        "iPhone",
        "2026-06-30 23:30:00 -0500",
        "2026-06-30 23:40:00 -0500",
        "250",
    );
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    assert_eq!(
        days.keys().copied().collect::<Vec<_>>(),
        vec![day(2026, 6, 30)],
        "the local day encoded in the timestamp wins"
    );
    assert_approx(days[&day(2026, 6, 30)].steps, 250.0, "steps");
}

#[test]
fn malformed_records_are_skipped_but_the_rest_still_import() {
    let body = [
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "not a date",
            "2026-06-29 08:10:00 -0500",
            "612",
        ),
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "2026-06-29 09:00:00 -0500",
            "2026-06-29 09:10:00 -0500",
            "not a number",
        ),
        record(
            "HKQuantityTypeIdentifierStepCount",
            "iPhone",
            "2026-06-29 10:00:00 -0500",
            "2026-06-29 10:10:00 -0500",
            "300",
        ),
    ]
    .join("\n");
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    assert_approx(
        days[&day(2026, 6, 29)].steps,
        300.0,
        "only the parseable record counts",
    );
}

#[test]
fn export_yielding_nothing_but_skips_is_a_parse_error() {
    let body = record(
        "HKQuantityTypeIdentifierStepCount",
        "iPhone",
        "not a date",
        "also not a date",
        "612",
    );
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    match import_apple_export(&path).unwrap_err() {
        HealthError::Parse { what, detail } => {
            assert_eq!(what, "Apple Health export");
            assert!(
                detail.contains("skipped"),
                "the error reports the skip count: {detail}"
            );
        }
        other => panic!("expected HealthError::Parse, got {other:?}"),
    }
}

#[test]
fn zip_without_an_export_entry_is_a_parse_error() {
    let dir = TempDir::new().unwrap();
    let path = write_zip(
        &dir,
        "export.zip",
        "apple_health_export/export_cda.xml",
        "<x/>",
    );
    // export_cda.xml is the clinical-records document, not the export — but as the
    // only shallow .xml it is the fallback candidate, so bury it deeper to prove the
    // no-candidate error too.
    let deep = write_zip(&dir, "deep.zip", "a/b/c/whatever.xml", "<x/>");
    match import_apple_export(&deep).unwrap_err() {
        HealthError::Parse { what, detail } => {
            assert_eq!(what, "Apple Health export");
            assert!(
                detail.contains("export.xml"),
                "the error names the expected entry: {detail}"
            );
        }
        other => panic!("expected HealthError::Parse, got {other:?}"),
    }
    // The lone-shallow-xml fallback picks export_cda.xml here; it then fails as
    // not-HealthData, which still names the input kind.
    assert!(matches!(
        import_apple_export(&path),
        Err(HealthError::Parse { .. })
    ));
}

#[test]
fn workout_duration_units_are_honored() {
    let body = [
        // 1800 sec = 30 min
        r#"<Workout workoutActivityType="HKWorkoutActivityTypeRunning" duration="1800" durationUnit="sec" sourceName="Watch" startDate="2026-06-29 09:00:00 -0500" endDate="2026-06-29 09:30:00 -0500"/>"#,
        // 0.5 hr = 30 min
        r#"<Workout workoutActivityType="HKWorkoutActivityTypeCycling" duration="0.5" durationUnit="hr" sourceName="Watch" startDate="2026-06-29 17:00:00 -0500" endDate="2026-06-29 17:30:00 -0500"/>"#,
    ]
    .join("\n");
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    let d = &days[&day(2026, 6, 29)];
    assert_eq!(d.workout_count, Some(2));
    assert_approx(d.workout_minutes, 60.0, "1800 sec + 0.5 hr = 60 min");
}

/// Serde shape guard: an imported day round-trips through the store's JSON without
/// inventing fields (None stays absent).
#[test]
fn imported_day_serializes_without_unmeasured_fields() {
    let body = record(
        "HKQuantityTypeIdentifierStepCount",
        "iPhone",
        "2026-06-29 10:00:00 -0500",
        "2026-06-29 10:10:00 -0500",
        "300",
    );
    let dir = TempDir::new().unwrap();
    let path = write_xml(&dir, "export.xml", &export(&body));
    let days = import_apple_export(&path).unwrap();
    let json = serde_json::to_string(&days[&day(2026, 6, 29)]).unwrap();
    assert_eq!(json, r#"{"steps":300.0}"#, "only measured fields serialize");
    let back: AppleDay = serde_json::from_str(&json).unwrap();
    assert_eq!(&back, &days[&day(2026, 6, 29)]);
}
