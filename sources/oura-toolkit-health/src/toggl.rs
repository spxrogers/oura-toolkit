//! Toggl Track importer: detailed-report CSV export → per-day [`TogglDay`] tracked-time
//! features. Toggl is the "how the day actually went" counterpart to the calendar's
//! plan; its strong time-blocking usage makes tracked blocks a high-signal load
//! feature.
//!
//! Input: the CSV of Toggl's detailed report export (columns include `Start date`,
//! `Start time`, `End date`, `End time`, `Duration`). Descriptions/projects/clients are
//! read past but never retained (same privacy rule as the calendar importer).
//!
//! Entries crossing midnight split at day boundaries, attributing each day its share.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::NaiveDate;

use crate::error::HealthError;
use crate::model::TogglDay;

/// Import a Toggl Track detailed-report CSV export.
pub fn import_toggl_csv(path: &Path) -> Result<BTreeMap<NaiveDate, TogglDay>, HealthError> {
    let _ = path;
    unimplemented!("Agent B: Toggl CSV parsing")
}
