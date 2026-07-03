//! oura-toolkit-health — the local day-grain health + life-context store, its file
//! importers, and the deterministic analog/capacity engine.
//!
//! This crate is the "importer archetype" half of the toolkit (feasibility doc §6):
//! providers with no cloud API (Apple Health, calendars, Toggl exports) enter here as
//! parsed FILES, land in one local store as per-day aggregates, and the engine answers
//! "weeks like this one" and "how much can this week take" over the result. The cloud
//! archetype (the generated Oura client) feeds the same store via `oura sync`.
//!
//! Hand-written by design: there is no spec to generate from and no HTTP transport in
//! this crate at all — parsers and statistics only. Everything is hermetic and
//! deterministic; nothing here touches the network.

pub mod apple;
pub mod calendar;
pub mod engine;
pub mod error;
pub mod model;
pub mod store;
pub mod toggl;

pub use error::HealthError;
pub use model::{AppleDay, CalendarDay, DayMap, DayRecord, OuraDay, SourceDay, TogglDay};
pub use store::{HealthStore, UpsertStats, DATA_DIR_NAME};
