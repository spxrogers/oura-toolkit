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
//!   would double-count the night.
//! - Quantity samples attribute to the day they START; daily means (heart rate, HRV,
//!   SpO2, temperature) weight each sample equally.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::NaiveDate;

use crate::error::HealthError;
use crate::model::AppleDay;

/// Import an Apple Health export: `.zip` (containing `apple_health_export/export.xml`)
/// or an already-extracted `export.xml`.
pub fn import_apple_export(path: &Path) -> Result<BTreeMap<NaiveDate, AppleDay>, HealthError> {
    let _ = path;
    unimplemented!("Agent A: Apple Health export parsing")
}
