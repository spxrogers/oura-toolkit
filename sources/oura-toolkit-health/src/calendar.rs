//! Calendar (.ics) importer: VEVENTs → per-day [`CalendarDay`] schedule features.
//!
//! Privacy rule (vision doc §3.1, enforced by the model): only DERIVED numbers leave
//! this module — SUMMARY/LOCATION/ATTENDEE lines are never parsed into anything that
//! outlives the parse.
//!
//! Scope (documented limitations, pinned by tests):
//! - Line unfolding, DTSTART/DTEND in date-time (floating, `Z`, or `TZID=`) and
//!   all-day (`VALUE=DATE`) forms.
//! - `TZID` times are treated as LOCAL wall-clock time (no tz database): context
//!   features describe the user's schedule shape, and home-timezone events dominate
//!   real calendars. `Z` times convert via the local offset.
//! - Recurrence: `RRULE` `FREQ=DAILY`/`FREQ=WEEKLY` expand (INTERVAL, BYDAY, UNTIL,
//!   COUNT, minus EXDATEs) within the import horizon; other FREQs count only their
//!   DTSTART occurrence.
//! - `STATUS:CANCELLED` events are skipped; multi-day timed events clamp to each day's
//!   boundaries.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::NaiveDate;

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
    let _ = (path, today);
    unimplemented!("Agent B: ics parsing")
}
