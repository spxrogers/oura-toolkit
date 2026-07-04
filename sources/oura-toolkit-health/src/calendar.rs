//! Calendar (.ics) importer: VEVENTs → per-day [`CalendarDay`] schedule features.
//!
//! Privacy rule (vision doc §3.1, enforced by the model): only DERIVED numbers leave
//! this module — SUMMARY/LOCATION/ATTENDEE lines are never parsed into anything that
//! outlives the parse. Concretely: a property whose name is not on the schedule-shape
//! allow-list (DTSTART, DTEND, DURATION, RRULE, EXDATE, STATUS) is skipped before its
//! value is ever copied out of the line buffer — no intermediate struct holds event
//! content.
//!
//! Scope (documented limitations, pinned by tests):
//! - Line unfolding, DTSTART/DTEND in date-time (floating, `Z`, or `TZID=`) and
//!   all-day (`VALUE=DATE`) forms.
//! - `TZID` times are treated as LOCAL wall-clock time (no tz database): context
//!   features describe the user's schedule shape, and home-timezone events dominate
//!   real calendars. `Z` times convert via the local offset.
//! - Timed durations come from DTEND, else from DURATION (`PT1H30M`, `P1DT2H`, `P1W`);
//!   an event with neither is skipped. All-day events (DTEND exclusive per RFC 5545)
//!   count only into `all_day_event_count` — no hours, no timed-event features.
//! - Recurrence: `RRULE` `FREQ=DAILY`/`FREQ=WEEKLY` expand (INTERVAL, BYDAY, UNTIL
//!   inclusive, COUNT, minus EXDATEs) within the import horizon; other FREQs
//!   (MONTHLY/YEARLY/…) count only their DTSTART occurrence. Per RFC 5545 §3.8.5.3
//!   DTSTART is ALWAYS the first occurrence, even when a weekly BYDAY list omits its
//!   weekday, and it consumes the first COUNT slot. COUNT is applied BEFORE EXDATE
//!   removal (an EXDATE never extends the series). An unparseable RRULE degrades to
//!   the DTSTART occurrence alone.
//! - The horizon (`today` + [`RECURRENCE_HORIZON_WEEKS`]) bounds recurrence EXPANSION
//!   only; a one-off event is imported wherever it falls (future weeks feed the
//!   capacity engine).
//! - `STATUS:CANCELLED` events are skipped; multi-day timed events clamp to each day's
//!   boundaries (the continuation day starts at minute 0, the previous day ends at
//!   minute 1440).
//! - Events with malformed dates (bad DTSTART/DTEND/DURATION/negative span) are
//!   skipped; if EVERY event in the file was malformed (and there was at least one),
//!   that is a [`HealthError::Parse`]. A file with neither `BEGIN:VCALENDAR` nor any
//!   VEVENT is not a calendar at all — also a parse error.

use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use chrono::{
    Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday,
};

use crate::error::HealthError;
use crate::model::CalendarDay;

/// Recurring events expand up to this many weeks past `today` — far enough to plan a
/// season ahead (`oura capacity`, `get_upcoming_load`), bounded so an UNTIL-less weekly
/// standup doesn't expand forever.
pub const RECURRENCE_HORIZON_WEEKS: u32 = 26;

/// Import a calendar export. `today` anchors the recurrence horizon.
pub fn import_ics(
    path: &Path,
    today: NaiveDate,
) -> Result<BTreeMap<NaiveDate, CalendarDay>, HealthError> {
    let raw = fs::read_to_string(path)?;
    let lines = unfold(&raw);
    let horizon = today + Duration::weeks(RECURRENCE_HORIZON_WEEKS as i64);

    let mut days: BTreeMap<NaiveDate, CalendarDay> = BTreeMap::new();
    let mut has_vcalendar = false;
    let mut events_seen: u32 = 0;
    let mut events_malformed: u32 = 0;

    let mut in_event = false;
    // Depth of components nested INSIDE the current VEVENT (VALARM…): their properties
    // (which include DURATION) must not leak into the event's own fields.
    let mut nest: u32 = 0;
    let mut ev = RawEvent::default();

    for line in &lines {
        let name = prop_name(line);
        if name.eq_ignore_ascii_case("BEGIN") {
            let comp = component_name(line);
            if comp.eq_ignore_ascii_case("VCALENDAR") {
                has_vcalendar = true;
            }
            if in_event {
                nest += 1;
            } else if comp.eq_ignore_ascii_case("VEVENT") {
                in_event = true;
                ev = RawEvent::default();
            }
            continue;
        }
        if name.eq_ignore_ascii_case("END") {
            if in_event {
                if nest > 0 {
                    nest -= 1;
                } else {
                    in_event = false;
                    events_seen += 1;
                    match std::mem::take(&mut ev).finalize() {
                        Finalized::Event(event) => expand_into(&mut days, &event, horizon),
                        Finalized::Malformed => events_malformed += 1,
                        Finalized::Skip => {}
                    }
                }
            }
            continue;
        }
        if !in_event || nest > 0 {
            continue;
        }
        // Allow-list gate: only schedule-shape properties get past this point, so
        // SUMMARY/LOCATION/DESCRIPTION/ATTENDEE values are never copied anywhere.
        if name.eq_ignore_ascii_case("DTSTART") {
            ev.saw_dtstart = true;
            ev.dtstart = parse_prop_time(line);
        } else if name.eq_ignore_ascii_case("DTEND") {
            ev.saw_dtend = true;
            ev.dtend = parse_prop_time(line);
        } else if name.eq_ignore_ascii_case("DURATION") {
            ev.saw_duration = true;
            ev.duration = split_params_value(line).and_then(|(_, v)| parse_ics_duration(v));
        } else if name.eq_ignore_ascii_case("RRULE") {
            ev.rrule = split_params_value(line).and_then(|(_, v)| parse_rrule(v));
        } else if name.eq_ignore_ascii_case("EXDATE") {
            if let Some((params, value)) = split_params_value(line) {
                // Comma-separated values, any datetime form; unparseable ones are
                // ignored (they could only over-remove, never leak content).
                for part in value.split(',') {
                    if let Some(t) = parse_ics_time(&params, part) {
                        ev.exdates.push(t);
                    }
                }
            }
        } else if name.eq_ignore_ascii_case("STATUS") {
            if let Some((_, v)) = split_params_value(line) {
                if v.trim().eq_ignore_ascii_case("CANCELLED") {
                    ev.cancelled = true;
                }
            }
        }
    }

    if !has_vcalendar && events_seen == 0 {
        return Err(HealthError::Parse {
            what: "calendar (.ics)",
            detail: "no BEGIN:VCALENDAR and no VEVENT found — not an iCalendar export".to_string(),
        });
    }
    if events_seen > 0 && events_malformed == events_seen {
        return Err(HealthError::Parse {
            what: "calendar (.ics)",
            detail: format!(
                "all {events_seen} events had malformed dates ({events_malformed} skipped)"
            ),
        });
    }
    Ok(days)
}

// ---------------------------------------------------------------------------
// Line-level parsing
// ---------------------------------------------------------------------------

/// RFC 5545 §3.1 unfolding: a physical line (CRLF or bare LF) beginning with a space
/// or tab continues the previous line; the line break plus that single whitespace
/// character are removed. Runs BEFORE any property parsing.
fn unfold(raw: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for physical in raw.split('\n') {
        let physical = physical.strip_suffix('\r').unwrap_or(physical);
        if let Some(cont) = physical
            .strip_prefix(' ')
            .or_else(|| physical.strip_prefix('\t'))
        {
            if let Some(last) = out.last_mut() {
                last.push_str(cont);
                continue;
            }
        }
        if !physical.is_empty() {
            out.push(physical.to_string());
        }
    }
    out
}

/// The property name: everything before the first `;` (parameter) or `:` (value).
fn prop_name(line: &str) -> &str {
    let end = line.find([';', ':']).unwrap_or(line.len());
    &line[..end]
}

/// For `BEGIN:`/`END:` lines: the component name after the colon.
fn component_name(line: &str) -> &str {
    line.split_once(':').map(|(_, c)| c.trim()).unwrap_or("")
}

/// Split a property line into (parameters, value) at the first `:` outside a quoted
/// parameter value (params like `ALTREP="http://…"` legally contain colons).
fn split_params_value(line: &str) -> Option<(Vec<&str>, &str)> {
    let mut in_quotes = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            ':' if !in_quotes => {
                let params = line[..i].split(';').skip(1).collect();
                return Some((params, &line[i + 1..]));
            }
            _ => {}
        }
    }
    None
}

/// A DTSTART/DTEND/EXDATE/UNTIL value, normalized: all timed values are LOCAL
/// wall-clock (`Z` converted via the local offset; `TZID` taken as-is — see module doc).
#[derive(Debug, Clone, Copy, PartialEq)]
enum IcsTime {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

fn parse_prop_time(line: &str) -> Option<IcsTime> {
    let (params, value) = split_params_value(line)?;
    parse_ics_time(&params, value)
}

fn parse_ics_time(params: &[&str], value: &str) -> Option<IcsTime> {
    let value = value.trim();
    let is_date = params
        .iter()
        .any(|p| p.trim().eq_ignore_ascii_case("VALUE=DATE"))
        || (value.len() == 8 && value.bytes().all(|b| b.is_ascii_digit()));
    if is_date {
        return NaiveDate::parse_from_str(value, "%Y%m%d")
            .ok()
            .map(IcsTime::Date);
    }
    if let Some(utc) = value.strip_suffix(['Z', 'z']) {
        let ndt = NaiveDateTime::parse_from_str(utc, "%Y%m%dT%H%M%S").ok()?;
        let local = Utc
            .from_utc_datetime(&ndt)
            .with_timezone(&Local)
            .naive_local();
        return Some(IcsTime::DateTime(local));
    }
    // Floating or TZID= — both treated as local wall clock.
    NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S")
        .ok()
        .map(IcsTime::DateTime)
}

/// RFC 5545 §3.3.6 duration subset: `P<n>W` or `P[<n>D][T[<n>H][<n>M][<n>S]]`.
/// Negative durations are rejected (meaningless as an event length).
fn parse_ics_duration(s: &str) -> Option<Duration> {
    let s = s.trim();
    let s = s.strip_prefix('+').unwrap_or(s);
    if s.starts_with('-') {
        return None;
    }
    let rest = s.strip_prefix(['P', 'p'])?;
    if rest.is_empty() {
        return None;
    }
    let (date_part, time_part) = match rest.split_once(['T', 't']) {
        Some((d, t)) => (d, Some(t)),
        None => (rest, None),
    };
    let mut secs: i64 = 0;
    let mut num = String::new();
    for c in date_part.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            let n: i64 = num.parse().ok()?;
            num.clear();
            match c.to_ascii_uppercase() {
                'W' => secs += n * 7 * 86_400,
                'D' => secs += n * 86_400,
                _ => return None,
            }
        }
    }
    if !num.is_empty() {
        return None; // trailing digits without a designator
    }
    if let Some(tp) = time_part {
        if tp.is_empty() {
            return None;
        }
        for c in tp.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                let n: i64 = num.parse().ok()?;
                num.clear();
                match c.to_ascii_uppercase() {
                    'H' => secs += n * 3_600,
                    'M' => secs += n * 60,
                    'S' => secs += n,
                    _ => return None,
                }
            }
        }
        if !num.is_empty() {
            return None;
        }
    }
    Some(Duration::seconds(secs))
}

// ---------------------------------------------------------------------------
// Recurrence
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum Freq {
    Daily,
    Weekly,
    /// MONTHLY/YEARLY/…: only the DTSTART occurrence is imported (module doc).
    Other,
}

#[derive(Debug)]
struct Rrule {
    freq: Freq,
    interval: u32,
    byday: Vec<Weekday>,
    count: Option<u32>,
    until: Option<IcsTime>,
}

fn parse_rrule(value: &str) -> Option<Rrule> {
    let mut freq = None;
    let mut interval: u32 = 1;
    let mut byday = Vec::new();
    let mut count = None;
    let mut until = None;
    for part in value.split(';') {
        let (k, v) = part.split_once('=')?;
        match k.trim().to_ascii_uppercase().as_str() {
            "FREQ" => {
                freq = Some(match v.trim().to_ascii_uppercase().as_str() {
                    "DAILY" => Freq::Daily,
                    "WEEKLY" => Freq::Weekly,
                    _ => Freq::Other,
                });
            }
            "INTERVAL" => interval = v.trim().parse().ok()?,
            "BYDAY" => {
                for tok in v.split(',') {
                    byday.push(parse_weekday(tok)?);
                }
            }
            "COUNT" => count = Some(v.trim().parse().ok()?),
            "UNTIL" => until = Some(parse_ics_time(&[], v)?),
            _ => {} // WKST, BYMONTH, … ignored
        }
    }
    Some(Rrule {
        freq: freq?,
        interval: interval.max(1),
        byday,
        count,
        until,
    })
}

fn parse_weekday(tok: &str) -> Option<Weekday> {
    // Strip a monthly-style ordinal prefix ("2MO", "-1FR") so it degrades gracefully.
    let letters = tok
        .trim()
        .trim_start_matches(|c: char| c == '+' || c == '-' || c.is_ascii_digit());
    match letters.to_ascii_uppercase().as_str() {
        "MO" => Some(Weekday::Mon),
        "TU" => Some(Weekday::Tue),
        "WE" => Some(Weekday::Wed),
        "TH" => Some(Weekday::Thu),
        "FR" => Some(Weekday::Fri),
        "SA" => Some(Weekday::Sat),
        "SU" => Some(Weekday::Sun),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Event assembly
// ---------------------------------------------------------------------------

/// Everything collected from one VEVENT — schedule-shape fields ONLY. There is no
/// field that could hold event content; that is the privacy rule made structural.
#[derive(Default)]
struct RawEvent {
    saw_dtstart: bool,
    dtstart: Option<IcsTime>,
    saw_dtend: bool,
    dtend: Option<IcsTime>,
    saw_duration: bool,
    duration: Option<Duration>,
    rrule: Option<Rrule>,
    exdates: Vec<IcsTime>,
    cancelled: bool,
}

enum EventKind {
    Timed(Duration),
    AllDay { days: i64 },
}

struct Event {
    start: IcsTime,
    kind: EventKind,
    rrule: Option<Rrule>,
    exdates: Vec<IcsTime>,
}

enum Finalized {
    Event(Event),
    /// Bad DTSTART / DTEND / DURATION / negative span — counts toward the
    /// all-events-malformed parse error.
    Malformed,
    /// Deliberate skips: STATUS:CANCELLED, or a timed event with neither DTEND nor
    /// DURATION (documented limitation, not a malformed file).
    Skip,
}

impl RawEvent {
    fn finalize(self) -> Finalized {
        if self.cancelled {
            return Finalized::Skip;
        }
        let Some(start) = self.dtstart else {
            // Missing entirely or present-but-unparseable: either way the event has
            // no usable anchor.
            return Finalized::Malformed;
        };
        if (self.saw_dtend && self.dtend.is_none())
            || (self.saw_duration && self.duration.is_none())
        {
            return Finalized::Malformed;
        }
        let kind = match start {
            IcsTime::Date(d) => {
                let days = match (self.dtend, self.duration) {
                    // DTEND is EXCLUSIVE for all-day events (RFC 5545 §3.8.2.2).
                    (Some(IcsTime::Date(e)), _) => (e - d).num_days(),
                    (Some(IcsTime::DateTime(e)), _) => (e.date() - d).num_days(),
                    (None, Some(dur)) => dur.num_days(),
                    (None, None) => 1,
                };
                if days < 0 {
                    return Finalized::Malformed;
                }
                EventKind::AllDay { days: days.max(1) }
            }
            IcsTime::DateTime(s) => {
                let dur = match (self.dtend, self.duration) {
                    (Some(IcsTime::DateTime(e)), _) => e - s,
                    // A VALUE=DATE end on a timed start is malformed input.
                    (Some(IcsTime::Date(_)), _) => return Finalized::Malformed,
                    (None, Some(dur)) => dur,
                    (None, None) => return Finalized::Skip,
                };
                if dur < Duration::zero() {
                    return Finalized::Malformed;
                }
                EventKind::Timed(dur)
            }
        };
        Finalized::Event(Event {
            start,
            kind,
            rrule: self.rrule,
            exdates: self.exdates,
        })
    }
}

// ---------------------------------------------------------------------------
// Expansion + per-day aggregation
// ---------------------------------------------------------------------------

/// Occurrence start DATES for an event (the time of day is constant per event).
/// DTSTART is always the first occurrence; rule-generated dates run from DTSTART
/// through `horizon`, bounded by UNTIL (inclusive) then truncated by COUNT — all
/// BEFORE EXDATE removal (see module doc).
fn occurrence_dates(event: &Event, horizon: NaiveDate) -> Vec<NaiveDate> {
    let (start_date, start_time) = match event.start {
        IcsTime::Date(d) => (d, None),
        IcsTime::DateTime(dt) => (dt.date(), Some(dt.time())),
    };
    let within_until = |d: NaiveDate, rule: &Rrule| -> bool {
        match rule.until {
            None => true,
            Some(IcsTime::Date(u)) => d <= u,
            Some(IcsTime::DateTime(u)) => match start_time {
                Some(t) => d.and_time(t) <= u,
                None => d <= u.date(),
            },
        }
    };

    let mut dates: BTreeSet<NaiveDate> = BTreeSet::new();
    dates.insert(start_date); // DTSTART is always an occurrence (RFC 5545 §3.8.5.3)
    if let Some(rule) = &event.rrule {
        match rule.freq {
            Freq::Other => {}
            Freq::Daily => {
                let mut k: i64 = 0;
                loop {
                    let d = start_date + Duration::days(k * rule.interval as i64);
                    if d > horizon || !within_until(d, rule) {
                        break;
                    }
                    dates.insert(d);
                    k += 1;
                }
            }
            Freq::Weekly if rule.byday.is_empty() => {
                let mut k: i64 = 0;
                loop {
                    let d = start_date + Duration::weeks(k * rule.interval as i64);
                    if d > horizon || !within_until(d, rule) {
                        break;
                    }
                    dates.insert(d);
                    k += 1;
                }
            }
            Freq::Weekly => {
                // DTSTART stays an occurrence even when BYDAY omits its weekday
                // (RFC 5545 §3.8.5.3 — see the module doc; pinned by test).
                // Weeks start Monday (WKST default); the rule fires on each listed
                // weekday of every INTERVAL-th week, starting from DTSTART's week.
                let week_start =
                    start_date - Duration::days(start_date.weekday().num_days_from_monday() as i64);
                let mut w: i64 = 0;
                loop {
                    let base = week_start + Duration::weeks(w * rule.interval as i64);
                    if base > horizon {
                        break;
                    }
                    for wd in &rule.byday {
                        let d = base + Duration::days(wd.num_days_from_monday() as i64);
                        if d >= start_date && d <= horizon && within_until(d, rule) {
                            dates.insert(d);
                        }
                    }
                    w += 1;
                }
            }
        }
        // COUNT counts from DTSTART inclusive, BEFORE EXDATE removal — the set is
        // chronological, and DTSTART is its first element by construction.
        let mut dates: Vec<NaiveDate> = dates.into_iter().collect();
        if let Some(c) = rule.count {
            dates.truncate(c as usize);
        }
        return remove_exdates(dates, start_time, &event.exdates);
    }
    remove_exdates(dates.into_iter().collect(), start_time, &event.exdates)
}

fn remove_exdates(
    mut dates: Vec<NaiveDate>,
    start_time: Option<NaiveTime>,
    exdates: &[IcsTime],
) -> Vec<NaiveDate> {
    dates.retain(|d| {
        !exdates.iter().any(|ex| match (ex, start_time) {
            // VALUE=DATE exdates match on the occurrence's date.
            (IcsTime::Date(xd), _) => xd == d,
            // Datetime exdates match the occurrence's exact start date+time…
            (IcsTime::DateTime(xdt), Some(t)) => *xdt == d.and_time(t),
            // …and degrade to date matching against all-day occurrences.
            (IcsTime::DateTime(xdt), None) => xdt.date() == *d,
        })
    });
    dates
}

fn expand_into(days: &mut BTreeMap<NaiveDate, CalendarDay>, event: &Event, horizon: NaiveDate) {
    let start_time = match event.start {
        IcsTime::Date(_) => None,
        IcsTime::DateTime(dt) => Some(dt.time()),
    };
    for occ_date in occurrence_dates(event, horizon) {
        match event.kind {
            EventKind::AllDay { days: n } => {
                for i in 0..n {
                    days.entry(occ_date + Duration::days(i))
                        .or_default()
                        .all_day_event_count += 1;
                }
            }
            EventKind::Timed(dur) => {
                let start = occ_date.and_time(start_time.expect("timed event has a time"));
                add_timed(days, start, dur);
            }
        }
    }
}

/// Clamp one timed occurrence at local midnights and fold each day's share into the
/// per-day features. `first_event_start_min`/`last_event_end_min` use the CLAMPED
/// segment: a continuation day starts at minute 0, the day before ends at 1440.
fn add_timed(days: &mut BTreeMap<NaiveDate, CalendarDay>, start: NaiveDateTime, dur: Duration) {
    let end = start + dur;
    let six_pm = NaiveTime::from_hms_opt(18, 0, 0).expect("valid time");
    let mut d = start.date();
    loop {
        let day_start = d.and_time(NaiveTime::MIN);
        let day_end = day_start + Duration::days(1);
        let seg_start = max(start, day_start);
        let seg_end = min(end, day_end);
        // A zero-length event still touches its start day (count + clock position),
        // but a segment that is empty only because of midnight clamping does not.
        if seg_end > seg_start || (dur.is_zero() && d == start.date()) {
            let day = days.entry(d).or_default();
            day.meeting_hours += (seg_end - seg_start).num_seconds() as f64 / 3600.0;
            day.event_count += 1;
            // Overlaps the 18:00–24:00 evening window (half-open; ending AT 18:00
            // does not count).
            if seg_end > d.and_time(six_pm) {
                day.evening_event_count += 1;
            }
            let start_min = (seg_start - day_start).num_minutes() as u32;
            let end_min = (seg_end - day_start).num_minutes() as u32;
            day.first_event_start_min = Some(
                day.first_event_start_min
                    .map_or(start_min, |m| m.min(start_min)),
            );
            day.last_event_end_min =
                Some(day.last_event_end_min.map_or(end_min, |m| m.max(end_min)));
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

    /// `today` for tests that don't exercise the horizon: far enough back that all
    /// fixture dates in July 2026 sit inside the 26-week window.
    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 7, 3).expect("valid date")
    }

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
    }

    fn import(ics: &str) -> BTreeMap<NaiveDate, CalendarDay> {
        try_import(ics).expect("fixture imports cleanly")
    }

    fn try_import(ics: &str) -> Result<BTreeMap<NaiveDate, CalendarDay>, HealthError> {
        let mut f = tempfile::NamedTempFile::new().expect("tempfile");
        f.write_all(ics.as_bytes()).expect("write fixture");
        import_ics(f.path(), today())
    }

    fn wrap(events: &str) -> String {
        format!("BEGIN:VCALENDAR\r\nVERSION:2.0\r\n{events}END:VCALENDAR\r\n")
    }

    #[test]
    fn multi_event_day_has_exact_features() {
        // 09:00–10:30 (1.5h) + 13:00–14:00 (1h): hand-computed sums and clock bounds.
        let ics = wrap(
            "BEGIN:VEVENT\r\nSUMMARY:Standup\r\nDTSTART:20260706T090000\r\nDTEND:20260706T103000\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nSUMMARY:1:1\r\nLOCATION:Room 4\r\nDTSTART:20260706T130000\r\nDTEND:20260706T140000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.len(), 1);
        let day = &days[&date(2026, 7, 6)];
        assert_eq!(day.meeting_hours, 2.5);
        assert_eq!(day.event_count, 2);
        assert_eq!(day.evening_event_count, 0);
        assert_eq!(day.all_day_event_count, 0);
        assert_eq!(day.first_event_start_min, Some(540));
        assert_eq!(day.last_event_end_min, Some(840));
    }

    #[test]
    fn evening_window_is_18_to_24_half_open() {
        // 17:30–18:30 overlaps the window; 16:00–18:00 ends AT 18:00 and does not.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T173000\r\nDTEND:20260706T183000\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nDTSTART:20260707T160000\r\nDTEND:20260707T180000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days[&date(2026, 7, 6)].evening_event_count, 1);
        assert_eq!(days[&date(2026, 7, 7)].evening_event_count, 0);
    }

    #[test]
    fn all_day_events_count_only_all_day_and_carry_no_hours() {
        // DTEND is exclusive: 06..08 covers the 6th and 7th, never the 8th.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART;VALUE=DATE:20260706\r\nDTEND;VALUE=DATE:20260708\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.len(), 2);
        for d in [date(2026, 7, 6), date(2026, 7, 7)] {
            let day = &days[&d];
            assert_eq!(day.all_day_event_count, 1);
            assert_eq!(day.meeting_hours, 0.0);
            assert_eq!(day.event_count, 0);
            assert_eq!(day.first_event_start_min, None);
            assert_eq!(day.last_event_end_min, None);
        }
        assert!(!days.contains_key(&date(2026, 7, 8)));
    }

    #[test]
    fn cancelled_events_are_skipped() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nSTATUS:CANCELLED\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\nEND:VEVENT\r\n",
        );
        assert!(import(&ics).is_empty());
    }

    #[test]
    fn multi_day_event_clamps_at_midnight_with_exact_shares() {
        // 06th 22:00 → 08th 01:00 = 27h: 2h + 24h + 1h.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T220000\r\nDTEND:20260708T010000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        let d6 = &days[&date(2026, 7, 6)];
        assert_eq!(d6.meeting_hours, 2.0);
        assert_eq!(d6.event_count, 1);
        assert_eq!(d6.evening_event_count, 1);
        assert_eq!(d6.first_event_start_min, Some(1320));
        assert_eq!(d6.last_event_end_min, Some(1440));
        let d7 = &days[&date(2026, 7, 7)];
        assert_eq!(d7.meeting_hours, 24.0);
        assert_eq!(d7.first_event_start_min, Some(0));
        assert_eq!(d7.last_event_end_min, Some(1440));
        assert_eq!(d7.evening_event_count, 1);
        let d8 = &days[&date(2026, 7, 8)];
        assert_eq!(d8.meeting_hours, 1.0);
        assert_eq!(d8.first_event_start_min, Some(0));
        assert_eq!(d8.last_event_end_min, Some(60));
        assert_eq!(d8.evening_event_count, 0);
    }

    #[test]
    fn weekly_byday_until_exdate_expand_to_exact_dates() {
        // Mondays + Wednesdays from Mon 2026-07-06, UNTIL Wed 2026-07-22 (inclusive),
        // minus the 2026-07-13 occurrence.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\n\
             RRULE:FREQ=WEEKLY;BYDAY=MO,WE;UNTIL=20260722T090000\r\n\
             EXDATE:20260713T090000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        let expected = [
            date(2026, 7, 6),
            date(2026, 7, 8),
            date(2026, 7, 15),
            date(2026, 7, 20),
            date(2026, 7, 22),
        ];
        assert_eq!(days.keys().copied().collect::<Vec<_>>(), expected);
        for d in expected {
            assert_eq!(days[&d].event_count, 1, "one occurrence on {d}");
            assert_eq!(days[&d].meeting_hours, 1.0);
        }
        assert!(!days.contains_key(&date(2026, 7, 13)), "EXDATE removed");
        assert!(!days.contains_key(&date(2026, 7, 27)), "beyond UNTIL");
    }

    #[test]
    fn daily_rrule_with_count_and_interval() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T093000\r\n\
             RRULE:FREQ=DAILY;INTERVAL=2;COUNT=3\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(
            days.keys().copied().collect::<Vec<_>>(),
            [date(2026, 7, 6), date(2026, 7, 8), date(2026, 7, 10)]
        );
        assert!(!days.contains_key(&date(2026, 7, 12)), "COUNT=3 exhausted");
    }

    #[test]
    fn count_is_applied_before_exdate_removal() {
        // COUNT=3 fixes the series at 06/07/08; EXDATE'ing the 7th must NOT pull the
        // 9th in as a replacement occurrence.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\n\
             RRULE:FREQ=DAILY;COUNT=3\r\nEXDATE:20260707T090000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(
            days.keys().copied().collect::<Vec<_>>(),
            [date(2026, 7, 6), date(2026, 7, 8)]
        );
    }

    #[test]
    fn untilless_weekly_rule_stops_at_the_horizon() {
        // today 2026-07-03 + 26 weeks = 2027-01-01. Mondays from 2026-07-06: the last
        // one inside the horizon is 2026-12-28; 2027-01-04 is the week after.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\n\
             RRULE:FREQ=WEEKLY\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert!(
            days.contains_key(&date(2026, 12, 28)),
            "last in-horizon Monday"
        );
        assert!(
            !days.contains_key(&date(2027, 1, 4)),
            "the week after today+{RECURRENCE_HORIZON_WEEKS}w must not expand"
        );
        assert_eq!(days.len(), 26, "Mondays 2026-07-06..=2026-12-28");
    }

    #[test]
    fn dtstart_is_an_occurrence_even_when_byday_omits_its_weekday() {
        // DTSTART Tue 2026-07-07 with BYDAY=WE: per the documented choice DTSTART is
        // always the first occurrence and consumes the first COUNT slot.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260707T090000\r\nDTEND:20260707T100000\r\n\
             RRULE:FREQ=WEEKLY;BYDAY=WE;COUNT=3\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(
            days.keys().copied().collect::<Vec<_>>(),
            [date(2026, 7, 7), date(2026, 7, 8), date(2026, 7, 15)]
        );
    }

    #[test]
    fn monthly_rrule_counts_only_the_dtstart_occurrence() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\n\
             RRULE:FREQ=MONTHLY\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.keys().copied().collect::<Vec<_>>(), [date(2026, 7, 6)]);
    }

    #[test]
    fn a_single_event_next_week_lands_in_the_map() {
        // Future events are the point of the horizon: the capacity engine reads them.
        let next_week = today() + Duration::weeks(1);
        let ics = wrap(&format!(
            "BEGIN:VEVENT\r\nDTSTART:{d}T100000\r\nDTEND:{d}T110000\r\nEND:VEVENT\r\n",
            d = next_week.format("%Y%m%d")
        ));
        let days = import(&ics);
        assert_eq!(days[&next_week].event_count, 1);
        assert_eq!(days[&next_week].meeting_hours, 1.0);
    }

    #[test]
    fn duration_property_covers_the_missing_dtend() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDURATION:PT1H30M\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nDTSTART:20260710T230000\r\nDURATION:P1DT2H\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days[&date(2026, 7, 6)].meeting_hours, 1.5);
        assert_eq!(days[&date(2026, 7, 6)].last_event_end_min, Some(630));
        // 23:00 + 26h → 1h on the 10th, 24h on the 11th, 1h on the 12th.
        assert_eq!(days[&date(2026, 7, 10)].meeting_hours, 1.0);
        assert_eq!(days[&date(2026, 7, 11)].meeting_hours, 24.0);
        assert_eq!(days[&date(2026, 7, 12)].meeting_hours, 1.0);
    }

    #[test]
    fn timed_event_with_neither_dtend_nor_duration_is_skipped_quietly() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nDTSTART:20260707T090000\r\nDTEND:20260707T100000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.keys().copied().collect::<Vec<_>>(), [date(2026, 7, 7)]);
    }

    #[test]
    fn utc_times_convert_via_the_local_offset() {
        // Robust to the machine's timezone: derive the expectation from the same
        // instant through chrono::Local, not a hardcoded offset.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T140000Z\r\nDTEND:20260706T150000Z\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        let local_start = Utc
            .with_ymd_and_hms(2026, 7, 6, 14, 0, 0)
            .single()
            .expect("valid instant")
            .with_timezone(&Local)
            .naive_local();
        let total_hours: f64 = days.values().map(|d| d.meeting_hours).sum();
        assert!(
            (total_hours - 1.0).abs() < 1e-9,
            "one hour total: {total_hours}"
        );
        let start_min =
            (local_start - local_start.date().and_time(NaiveTime::MIN)).num_minutes() as u32;
        assert_eq!(
            days[&local_start.date()].first_event_start_min,
            Some(start_min),
            "starts at the locally-converted wall-clock minute"
        );
    }

    #[test]
    fn tzid_times_are_taken_as_local_wall_clock() {
        // Documented limitation: no tz database — the wall-clock time IS the local
        // time, no matter which machine runs the import.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART;TZID=America/Chicago:20260706T090000\r\n\
             DTEND;TZID=America/Chicago:20260706T101500\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        let day = &days[&date(2026, 7, 6)];
        assert_eq!(day.first_event_start_min, Some(540));
        assert_eq!(day.last_event_end_min, Some(615));
        assert_eq!(day.meeting_hours, 1.25);
    }

    #[test]
    fn folded_lines_unfold_before_parsing() {
        // The DTSTART itself is folded mid-value (CRLF + space), and a long folded
        // SUMMARY sits in between — both must survive unfolding.
        let ics = wrap(
            "BEGIN:VEVENT\r\nSUMMARY:A very long meeting title that an exporter\r\n \
             decided to fold across physical lines\r\nDTSTART:20260706T\r\n 090000\r\n\
             DTEND:20260706T100000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days[&date(2026, 7, 6)].first_event_start_min, Some(540));
        assert_eq!(days[&date(2026, 7, 6)].meeting_hours, 1.0);
    }

    #[test]
    fn no_event_content_survives_into_the_output() {
        // Privacy rule: serialize the whole result and prove no fragment of
        // SUMMARY/LOCATION/DESCRIPTION/ATTENDEE text is anywhere in it.
        let ics = wrap(
            "BEGIN:VEVENT\r\nSUMMARY:SECRET_MEETING_TITLE\r\nLOCATION:HIDDEN_BUNKER\r\n\
             DESCRIPTION:CLASSIFIED_AGENDA\r\nATTENDEE:mailto:mole@example.com\r\n\
             DTSTART:20260706T090000\r\nDTEND:20260706T100000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.len(), 1, "the event itself is imported");
        let json = serde_json::to_string(&days).expect("serializes");
        for leak in [
            "SECRET_MEETING_TITLE",
            "HIDDEN_BUNKER",
            "CLASSIFIED_AGENDA",
            "mole@example.com",
        ] {
            assert!(
                !json.contains(leak),
                "event content leaked into output: {leak}"
            );
        }
    }

    #[test]
    fn not_a_calendar_file_is_a_parse_error() {
        let err = try_import("just some\nrandom text\n").expect_err("must not parse");
        match err {
            HealthError::Parse { what, detail } => {
                assert_eq!(what, "calendar (.ics)");
                assert!(
                    detail.contains("not an iCalendar export"),
                    "detail: {detail}"
                );
            }
            other => panic!("expected Parse error, got: {other}"),
        }
    }

    #[test]
    fn all_events_malformed_is_a_parse_error_naming_the_count() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:garbage\r\nDTEND:20260706T100000\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nDTEND:20260707T100000\r\nEND:VEVENT\r\n",
        );
        let err = try_import(&ics).expect_err("must not parse");
        match err {
            HealthError::Parse { what, detail } => {
                assert_eq!(what, "calendar (.ics)");
                assert!(
                    detail.contains('2'),
                    "detail counts the skipped events: {detail}"
                );
            }
            other => panic!("expected Parse error, got: {other}"),
        }
    }

    #[test]
    fn one_malformed_event_among_good_ones_is_skipped_not_fatal() {
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:garbage\r\nDTEND:20260706T100000\r\nEND:VEVENT\r\n\
             BEGIN:VEVENT\r\nDTSTART:20260707T090000\r\nDTEND:20260707T100000\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days.keys().copied().collect::<Vec<_>>(), [date(2026, 7, 7)]);
    }

    #[test]
    fn valarm_duration_does_not_leak_into_the_event() {
        // A VALARM nested in the VEVENT carries its own DURATION; the event's real
        // length comes from DTEND, not the alarm.
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART:20260706T090000\r\nDTEND:20260706T100000\r\n\
             BEGIN:VALARM\r\nTRIGGER:-PT15M\r\nDURATION:PT5M\r\nACTION:DISPLAY\r\nEND:VALARM\r\n\
             END:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(days[&date(2026, 7, 6)].meeting_hours, 1.0);
    }

    #[test]
    fn recurring_all_day_event_with_date_exdate() {
        // Weekly all-day event, one date EXDATE'd out (VALUE=DATE match-on-date rule).
        let ics = wrap(
            "BEGIN:VEVENT\r\nDTSTART;VALUE=DATE:20260706\r\n\
             RRULE:FREQ=WEEKLY;COUNT=3\r\nEXDATE;VALUE=DATE:20260713\r\nEND:VEVENT\r\n",
        );
        let days = import(&ics);
        assert_eq!(
            days.keys().copied().collect::<Vec<_>>(),
            [date(2026, 7, 6), date(2026, 7, 20)]
        );
        assert_eq!(days[&date(2026, 7, 6)].all_day_event_count, 1);
        assert_eq!(days[&date(2026, 7, 6)].meeting_hours, 0.0);
    }

    #[test]
    fn bare_lf_line_endings_parse_too() {
        let ics = "BEGIN:VCALENDAR\nBEGIN:VEVENT\nDTSTART:20260706T090000\nDTEND:20260706T100000\nEND:VEVENT\nEND:VCALENDAR\n";
        let days = import(ics);
        assert_eq!(days[&date(2026, 7, 6)].meeting_hours, 1.0);
    }
}
