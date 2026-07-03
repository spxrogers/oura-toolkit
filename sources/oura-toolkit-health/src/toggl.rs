//! Toggl Track importer: detailed-report CSV export → per-day [`TogglDay`] tracked-time
//! features. Toggl is the "how the day actually went" counterpart to the calendar's
//! plan; its strong time-blocking usage makes tracked blocks a high-signal load
//! feature.
//!
//! Input: the CSV of Toggl's detailed report export. Column ORDER varies across export
//! versions, so the required columns (`Start date`, `Start time`, `End date`,
//! `End time`, `Duration`) are resolved BY HEADER NAME, case-insensitively; a missing
//! one is a [`HealthError::Parse`] naming it. Descriptions/projects/clients/users are
//! read past by position but never retained (same privacy rule as the calendar
//! importer — only derived numbers leave this module).
//!
//! Entries crossing midnight split at day boundaries, attributing each day its share:
//! `entry_count` increments on every day touched, `longest_block_minutes` compares the
//! per-day CLAMPED share, the continuation day starts at minute 0 and the previous day
//! ends at minute 1440. Duration prefers `end − start` and falls back to the `Duration`
//! column (`HH:MM:SS`) when the end fields are empty. Malformed rows (bad dates, end
//! before start) are skipped; if EVERY row was malformed (and there was at least one),
//! that is a parse error. A header-only export yields an empty map.

use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::path::Path;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};

use crate::error::HealthError;
use crate::model::TogglDay;

/// The columns the importer needs; everything else in the export is ignored.
const REQUIRED_COLUMNS: [&str; 5] = [
    "Start date",
    "Start time",
    "End date",
    "End time",
    "Duration",
];

/// Import a Toggl Track detailed-report CSV export.
pub fn import_toggl_csv(path: &Path) -> Result<BTreeMap<NaiveDate, TogglDay>, HealthError> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true) // short rows become malformed-row skips, not hard errors
        .from_path(path)
        .map_err(csv_error)?;
    let headers = rdr.headers().map_err(csv_error)?.clone();

    let mut idx = [0usize; REQUIRED_COLUMNS.len()];
    let mut missing = Vec::new();
    for (slot, name) in idx.iter_mut().zip(REQUIRED_COLUMNS) {
        match headers
            .iter()
            .position(|h| h.trim().eq_ignore_ascii_case(name))
        {
            Some(i) => *slot = i,
            None => missing.push(name),
        }
    }
    if !missing.is_empty() {
        return Err(HealthError::Parse {
            what: "Toggl CSV",
            detail: format!("missing required column(s): {}", missing.join(", ")),
        });
    }
    let [start_date_i, start_time_i, end_date_i, end_time_i, duration_i] = idx;

    let mut days: BTreeMap<NaiveDate, TogglDay> = BTreeMap::new();
    let mut rows_seen: u32 = 0;
    let mut rows_malformed: u32 = 0;

    for record in rdr.records() {
        rows_seen += 1;
        let Ok(record) = record else {
            rows_malformed += 1;
            continue;
        };
        let field = |i: usize| record.get(i).unwrap_or("").trim();

        let Some(start) = parse_date_time(field(start_date_i), field(start_time_i)) else {
            rows_malformed += 1;
            continue;
        };
        // Prefer end − start; fall back to the Duration column when end is missing.
        let end = if !field(end_date_i).is_empty() && !field(end_time_i).is_empty() {
            parse_date_time(field(end_date_i), field(end_time_i))
        } else {
            parse_hms(field(duration_i)).map(|d| start + d)
        };
        let Some(end) = end else {
            rows_malformed += 1;
            continue;
        };
        if end < start {
            rows_malformed += 1;
            continue;
        }
        add_entry(&mut days, start, end);
    }

    if rows_seen > 0 && rows_malformed == rows_seen {
        return Err(HealthError::Parse {
            what: "Toggl CSV",
            detail: format!("all {rows_seen} data rows were malformed ({rows_malformed} skipped)"),
        });
    }
    Ok(days)
}

/// I/O problems keep their identity; anything else about the file is a parse problem.
fn csv_error(e: csv::Error) -> HealthError {
    if e.is_io_error() {
        match e.into_kind() {
            csv::ErrorKind::Io(io) => HealthError::Io(io),
            _ => unreachable!("is_io_error guarantees the Io kind"),
        }
    } else {
        HealthError::Parse {
            what: "Toggl CSV",
            detail: e.to_string(),
        }
    }
}

fn parse_date_time(date: &str, time: &str) -> Option<NaiveDateTime> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let t = NaiveTime::parse_from_str(time, "%H:%M:%S").ok()?;
    Some(d.and_time(t))
}

/// Toggl's `Duration` column: `HH:MM:SS`, where hours may exceed two digits.
fn parse_hms(s: &str) -> Option<Duration> {
    let mut parts = s.trim().split(':');
    let (h, m, sec) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let h: u32 = h.parse().ok()?;
    let m: u32 = m.parse().ok()?;
    let sec: u32 = sec.parse().ok()?;
    if m > 59 || sec > 59 {
        return None;
    }
    Some(Duration::seconds(
        i64::from(h) * 3_600 + i64::from(m) * 60 + i64::from(sec),
    ))
}

/// Clamp one entry at local midnights and fold each day's share into the per-day
/// features (same clamping semantics as the calendar importer's timed events).
fn add_entry(days: &mut BTreeMap<NaiveDate, TogglDay>, start: NaiveDateTime, end: NaiveDateTime) {
    let mut d = start.date();
    loop {
        let day_start = d.and_time(NaiveTime::MIN);
        let day_end = day_start + Duration::days(1);
        let seg_start = max(start, day_start);
        let seg_end = min(end, day_end);
        // A zero-length entry still touches its start day; an empty segment created
        // only by midnight clamping does not.
        if seg_end > seg_start || (end == start && d == start.date()) {
            let day = days.entry(d).or_default();
            let secs = (seg_end - seg_start).num_seconds() as f64;
            day.tracked_hours += secs / 3_600.0;
            day.entry_count += 1;
            day.longest_block_minutes = day.longest_block_minutes.max(secs / 60.0);
            let start_min = (seg_start - day_start).num_minutes() as u32;
            let end_min = (seg_end - day_start).num_minutes() as u32;
            day.first_start_min = Some(day.first_start_min.map_or(start_min, |m| m.min(start_min)));
            day.last_end_min = Some(day.last_end_min.map_or(end_min, |m| m.max(end_min)));
        }
        if end <= day_end {
            break;
        }
        d = d.succ_opt().expect("date within chrono range");
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    const HEADER: &str =
        "User,Email,Client,Project,Task,Description,Billable,Start date,Start time,End date,End time,Duration,Tags,Amount ()";

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
    }

    fn import(csv: &str) -> BTreeMap<NaiveDate, TogglDay> {
        try_import(csv).expect("fixture imports cleanly")
    }

    fn try_import(csv: &str) -> Result<BTreeMap<NaiveDate, TogglDay>, HealthError> {
        let mut f = tempfile::NamedTempFile::new().expect("tempfile");
        f.write_all(csv.as_bytes()).expect("write fixture");
        import_toggl_csv(f.path())
    }

    /// A row in the canonical `HEADER` column order.
    fn row(desc: &str, sd: &str, st: &str, ed: &str, et: &str, dur: &str) -> String {
        format!("Me,me@example.com,Acme,Deep Work,,{desc},No,{sd},{st},{ed},{et},{dur},,\n")
    }

    #[test]
    fn multi_entry_day_has_exact_sums() {
        // 09:00–10:30 (90 min) + 13:00–14:15 (75 min): hand-computed features.
        let csv = format!(
            "{HEADER}\n{}{}",
            row(
                "write",
                "2026-07-06",
                "09:00:00",
                "2026-07-06",
                "10:30:00",
                "01:30:00"
            ),
            row(
                "review",
                "2026-07-06",
                "13:00:00",
                "2026-07-06",
                "14:15:00",
                "01:15:00"
            ),
        );
        let days = import(&csv);
        assert_eq!(days.len(), 1);
        let day = &days[&date(2026, 7, 6)];
        assert_eq!(day.tracked_hours, 2.75);
        assert_eq!(day.entry_count, 2);
        assert_eq!(day.longest_block_minutes, 90.0);
        assert_eq!(day.first_start_min, Some(540));
        assert_eq!(day.last_end_min, Some(855));
    }

    #[test]
    fn midnight_crossing_entry_splits_with_exact_shares() {
        // 22:30 → 01:00 next day: 1.5h on day one (ends at 1440), 1h on day two
        // (starts at 0); the entry counts on BOTH days.
        let csv = format!(
            "{HEADER}\n{}",
            row(
                "night",
                "2026-07-06",
                "22:30:00",
                "2026-07-07",
                "01:00:00",
                "02:30:00"
            ),
        );
        let days = import(&csv);
        let d6 = &days[&date(2026, 7, 6)];
        assert_eq!(d6.tracked_hours, 1.5);
        assert_eq!(d6.entry_count, 1);
        assert_eq!(
            d6.longest_block_minutes, 90.0,
            "clamped share, not the full entry"
        );
        assert_eq!(d6.first_start_min, Some(1350));
        assert_eq!(d6.last_end_min, Some(1440));
        let d7 = &days[&date(2026, 7, 7)];
        assert_eq!(d7.tracked_hours, 1.0);
        assert_eq!(d7.entry_count, 1);
        assert_eq!(d7.longest_block_minutes, 60.0);
        assert_eq!(d7.first_start_min, Some(0));
        assert_eq!(d7.last_end_min, Some(60));
    }

    #[test]
    fn longest_block_is_the_max_single_share() {
        let csv = format!(
            "{HEADER}\n{}{}",
            row(
                "short",
                "2026-07-06",
                "09:00:00",
                "2026-07-06",
                "09:30:00",
                "00:30:00"
            ),
            row(
                "long",
                "2026-07-06",
                "10:00:00",
                "2026-07-06",
                "12:00:00",
                "02:00:00"
            ),
        );
        let days = import(&csv);
        assert_eq!(days[&date(2026, 7, 6)].longest_block_minutes, 120.0);
        assert_eq!(days[&date(2026, 7, 6)].tracked_hours, 2.5);
    }

    #[test]
    fn columns_resolve_by_header_name_case_insensitively_in_any_order() {
        // Different export version: reordered columns, lowercased header names.
        let csv = "duration,end time,end date,description,start time,start date\n\
                   01:00:00,15:00:00,2026-07-06,secret work,14:00:00,2026-07-06\n";
        let days = import(csv);
        let day = &days[&date(2026, 7, 6)];
        assert_eq!(day.tracked_hours, 1.0);
        assert_eq!(day.first_start_min, Some(840));
        assert_eq!(day.last_end_min, Some(900));
    }

    #[test]
    fn missing_required_column_is_a_parse_error_naming_it() {
        let csv = "User,Description,Start date,Start time,End date,Duration\n\
                   Me,x,2026-07-06,09:00:00,2026-07-06,01:00:00\n";
        let err = try_import(csv).expect_err("must not parse");
        match err {
            HealthError::Parse { what, detail } => {
                assert_eq!(what, "Toggl CSV");
                assert!(
                    detail.contains("End time"),
                    "names the missing column: {detail}"
                );
            }
            other => panic!("expected Parse error, got: {other}"),
        }
    }

    #[test]
    fn duration_column_covers_a_missing_end() {
        let csv = format!(
            "{HEADER}\n{}",
            row(
                "running timer",
                "2026-07-06",
                "09:00:00",
                "",
                "",
                "01:30:00"
            ),
        );
        let days = import(&csv);
        assert_eq!(days[&date(2026, 7, 6)].tracked_hours, 1.5);
        assert_eq!(days[&date(2026, 7, 6)].last_end_min, Some(630));
    }

    #[test]
    fn descriptions_are_never_retained_even_hostile_ones() {
        // Privacy rule: a description carrying ANSI escapes / control bytes must leave
        // no trace — not the text, not the escape bytes — in the serialized output.
        let csv = format!(
            "{HEADER}\n{}",
            row(
                "\u{1b}[31mTOPSECRET_CLIENT_WORK\u{1b}[0m\u{7}",
                "2026-07-06",
                "09:00:00",
                "2026-07-06",
                "10:00:00",
                "01:00:00"
            ),
        );
        let days = import(&csv);
        assert_eq!(
            days[&date(2026, 7, 6)].entry_count,
            1,
            "the entry itself imports"
        );
        let json = serde_json::to_string(&days).expect("serializes");
        assert!(
            !json.contains("TOPSECRET_CLIENT_WORK"),
            "description text leaked"
        );
        assert!(!json.contains('\u{1b}'), "escape bytes leaked");
        assert!(!json.contains("\\u001b"), "escaped escape bytes leaked");
    }

    #[test]
    fn malformed_rows_are_skipped_but_good_rows_import() {
        let csv = format!(
            "{HEADER}\n{}{}{}",
            row(
                "bad date",
                "not-a-date",
                "09:00:00",
                "2026-07-06",
                "10:00:00",
                "01:00:00"
            ),
            // End before start: skipped, counted.
            row(
                "backwards",
                "2026-07-06",
                "10:00:00",
                "2026-07-06",
                "09:00:00",
                "01:00:00"
            ),
            row(
                "good",
                "2026-07-07",
                "09:00:00",
                "2026-07-07",
                "10:00:00",
                "01:00:00"
            ),
        );
        let days = import(&csv);
        assert_eq!(days.keys().copied().collect::<Vec<_>>(), [date(2026, 7, 7)]);
        assert_eq!(days[&date(2026, 7, 7)].tracked_hours, 1.0);
    }

    #[test]
    fn all_rows_malformed_is_a_parse_error() {
        let csv = format!(
            "{HEADER}\n{}",
            row(
                "bad",
                "garbage",
                "09:00:00",
                "2026-07-06",
                "10:00:00",
                "01:00:00"
            ),
        );
        let err = try_import(&csv).expect_err("must not parse");
        match err {
            HealthError::Parse { what, detail } => {
                assert_eq!(what, "Toggl CSV");
                assert!(detail.contains("all 1 data rows"), "detail: {detail}");
            }
            other => panic!("expected Parse error, got: {other}"),
        }
    }

    #[test]
    fn zero_data_rows_yield_an_empty_map() {
        let days = import(&format!("{HEADER}\n"));
        assert!(days.is_empty());
    }
}
