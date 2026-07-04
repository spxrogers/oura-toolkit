//! The local day-grain health store: one JSON record file at a fixed,
//! invocation-independent per-platform DATA path (a sibling of the auth crate's config
//! store, same discipline, different XDG base — imported health data is not a
//! credential, but it gets the same protections):
//!
//! - **Unix (incl. macOS):** `$XDG_DATA_HOME/oura-toolkit/` →
//!   `~/.local/share/oura-toolkit/`.
//! - **Windows:** `%LOCALAPPDATA%\oura-toolkit\data\` (under the config dir's parent —
//!   Local, not Roaming, for the same don't-sync-off-the-machine reason as tokens).
//!
//! Same file protections as the token store: `0700` dir + `0600` records + atomic
//! uniquely-named-temp + rename writes on Unix; `%LOCALAPPDATA%` ACLs on Windows.
//! Writes run under the same advisory `.lock` discipline so a CLI import and a
//! long-running MCP server can share the store.
//!
//! The store is a REBUILDABLE CACHE of imports — sources of truth stay in Apple
//! Health / Oura's cloud / the calendar. Losing it costs a re-import; that is a
//! documented property (it keeps at-rest encryption addable later without migration).
//!
//! Day-grain aggregates ONLY (per-day numbers, ~a few KB per year of history) — raw
//! samples are never retained, so the single-JSON-file format stays comfortably small
//! and `sqlite` remains an unneeded dependency until a real row-count demands it
//! (Phase-0 decision, revising the vision doc's SQLite lean; recorded there).

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::error::HealthError;
use crate::model::{DayMap, SourceDay};

/// The locked data-directory name — same name as the auth crate's `APP_DIR_NAME`
/// (CLAUDE.md → NAMING), pinned by the docs tripwire for the README's store-path claim.
pub const DATA_DIR_NAME: &str = "oura-toolkit";

/// The record file inside the data dir.
pub const STORE_FILE: &str = "days.json";

/// On-disk format version; bumped only with a migration story.
pub const STORE_VERSION: u32 = 1;

/// On-disk shape: a version header around the day map.
#[derive(Serialize, Deserialize)]
struct StoreFile {
    version: u32,
    days: DayMap,
}

/// Result of an upsert: how the import changed the store, for the CLI's summary line.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct UpsertStats {
    /// Days that did not exist in the store at all.
    pub added: u32,
    /// Days that existed and whose slot content changed.
    pub updated: u32,
    /// Days already present with identical slot content (idempotent re-import).
    pub unchanged: u32,
}

/// Handle to the on-disk store directory.
#[derive(Debug, Clone)]
pub struct HealthStore {
    dir: PathBuf,
}

/// An exclusive advisory lock on the store, released on drop.
#[must_use = "the store lock releases as soon as the guard is dropped — bind it for the critical section"]
struct StoreLock(fs::File);

impl Drop for StoreLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

impl HealthStore {
    /// Store at the default per-platform data location.
    pub fn open_default() -> Result<Self, HealthError> {
        Ok(Self { dir: data_dir()? })
    }

    /// Store rooted at an explicit directory (used by tests).
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// The store directory.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn record_path(&self) -> PathBuf {
        self.dir.join(STORE_FILE)
    }

    /// Load every stored day. A store that has never been written is empty, not an error.
    pub fn load(&self) -> Result<DayMap, HealthError> {
        match fs::read(self.record_path()) {
            Ok(bytes) => {
                let file: StoreFile = serde_json::from_slice(&bytes)?;
                if file.version != STORE_VERSION {
                    return Err(HealthError::Format(format!(
                        "store version {} is newer than this build supports ({STORE_VERSION}) — \
                         upgrade oura-toolkit",
                        file.version
                    )));
                }
                Ok(file.days)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(DayMap::new()),
            Err(e) => Err(e.into()),
        }
    }

    /// Merge one source's days into the store: for each date, REPLACE that source's slot
    /// wholesale (source-level idempotency; other sources' slots are untouched). Runs
    /// read→merge→write under the exclusive store lock so a concurrent import in another
    /// process cannot lose either side's days.
    pub fn upsert<S: SourceDay>(
        &self,
        days: BTreeMap<NaiveDate, S>,
    ) -> Result<UpsertStats, HealthError> {
        self.ensure_dir()?;
        let _lock = self.lock_exclusive()?;
        let mut all = self.load()?;
        let mut stats = UpsertStats::default();
        for (date, incoming) in days {
            let entry = all.entry(date).or_default();
            let slot = S::slot(entry);
            match slot {
                Some(existing) if *existing == incoming => stats.unchanged += 1,
                Some(_) => {
                    *slot = Some(incoming);
                    stats.updated += 1;
                }
                None => {
                    *slot = Some(incoming);
                    // "added" counts days new to the store; a new slot on an existing
                    // day (any other source's, or user-logged habits) is an update.
                    if entry.oura.is_some() as u8
                        + entry.apple.is_some() as u8
                        + entry.calendar.is_some() as u8
                        + entry.toggl.is_some() as u8
                        + entry.habits.is_some() as u8
                        == 1
                    {
                        stats.added += 1;
                    } else {
                        stats.updated += 1;
                    }
                }
            }
        }
        self.save(&all)?;
        Ok(stats)
    }

    /// Mark `habit` done on `date` (name normalized via
    /// [`crate::habits::normalize_habit_name`]). Read→modify→write under the exclusive
    /// store lock — logging one habit can never clobber the day's other habits or any
    /// source slot. Returns the canonical name and whether the log is NEW (false = it
    /// was already logged; idempotent).
    pub fn log_habit(&self, date: NaiveDate, habit: &str) -> Result<(String, bool), HealthError> {
        let name = crate::habits::normalize_habit_name(habit)?;
        self.ensure_dir()?;
        let _lock = self.lock_exclusive()?;
        let mut all = self.load()?;
        let day = all.entry(date).or_default();
        let changed = day
            .habits
            .get_or_insert_with(crate::model::HabitsDay::default)
            .done
            .insert(name.clone());
        if changed {
            self.save(&all)?;
        }
        Ok((name, changed))
    }

    /// Remove `habit` from `date` (the undo of [`Self::log_habit`]). Returns the
    /// canonical name and whether anything was actually removed. An emptied habits set
    /// is dropped from the record (and an emptied record from the store) so undo leaves
    /// no husks behind.
    pub fn unlog_habit(&self, date: NaiveDate, habit: &str) -> Result<(String, bool), HealthError> {
        let name = crate::habits::normalize_habit_name(habit)?;
        self.ensure_dir()?;
        let _lock = self.lock_exclusive()?;
        let mut all = self.load()?;
        let mut changed = false;
        if let Some(day) = all.get_mut(&date) {
            if let Some(h) = &mut day.habits {
                changed = h.done.remove(&name);
                if h.done.is_empty() {
                    day.habits = None;
                }
            }
            if day.is_empty() {
                all.remove(&date);
            }
        }
        if changed {
            self.save(&all)?;
        }
        Ok((name, changed))
    }

    fn save(&self, days: &DayMap) -> Result<(), HealthError> {
        self.ensure_dir()?;
        let file = StoreFile {
            version: STORE_VERSION,
            days: days.clone(),
        };
        let data = serde_json::to_vec_pretty(&file)?;
        write_secure(&self.record_path(), &data)?;
        Ok(())
    }

    /// Blocking exclusive advisory lock, same semantics/caveats as the token store's
    /// (cooperative `flock`/`LockFileEx` on a `.lock` file; inode continuity assumed).
    fn lock_exclusive(&self) -> Result<StoreLock, HealthError> {
        self.ensure_dir()?;
        let file = open_owner_only(&self.dir.join(".lock"))?;
        file.lock().map_err(HealthError::Io)?;
        Ok(StoreLock(file))
    }

    fn ensure_dir(&self) -> Result<(), HealthError> {
        fs::create_dir_all(&self.dir)?;
        set_dir_private(&self.dir)?;
        Ok(())
    }
}

/// The fixed, invocation-independent data dir (module doc has the per-platform story).
fn data_dir() -> Result<PathBuf, HealthError> {
    data_dir_from(&|key| std::env::var(key).ok())
}

/// Testable core of [`data_dir`]: injected env lookup, no racy `env::set_var`. Empty and
/// relative values are ignored (XDG spec; a relative base would make where health data
/// lands depend on the process cwd).
fn data_dir_from(env: &dyn Fn(&str) -> Option<String>) -> Result<PathBuf, HealthError> {
    let usable = |key: &str| -> Option<PathBuf> {
        env(key)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
    };

    #[cfg(windows)]
    let dir = usable("LOCALAPPDATA").map(|base| base.join(DATA_DIR_NAME).join("data"));

    #[cfg(not(windows))]
    let dir = usable("XDG_DATA_HOME")
        .map(|xdg| xdg.join(DATA_DIR_NAME))
        .or_else(|| {
            usable("HOME").map(|home| home.join(".local").join("share").join(DATA_DIR_NAME))
        });

    dir.ok_or(HealthError::NoDataDir)
}

/// If the store directory resolves inside a known cloud-sync root, the health data would
/// silently replicate off the machine — the exact leak the vision doc's threat model
/// ranks highest. Returns the warning the CLI prints to stderr (never a hard error: the
/// user may have deliberately chosen it, and refusing would strand their data).
pub fn sync_root_warning(dir: &Path) -> Option<String> {
    // Component names (lowercased) that identify the major sync products' roots.
    const SYNC_ROOTS: &[&str] = &[
        "dropbox",
        "onedrive",
        "google drive",
        "googledrive",
        "com~apple~clouddocs", // iCloud Drive's on-disk container
        "mobile documents",    // ~/Library/Mobile Documents (iCloud)
    ];
    for component in dir.components() {
        let name = component.as_os_str().to_string_lossy().to_lowercase();
        if SYNC_ROOTS.iter().any(|root| name == *root) {
            return Some(format!(
                "warning: the health data store ({}) is inside a cloud-synced folder — \
                 imported health data will replicate off this machine. Point \
                 XDG_DATA_HOME somewhere local to keep it device-only.",
                dir.display()
            ));
        }
    }
    None
}

/// Open (creating if needed) with owner-only perms where supported.
#[cfg(unix)]
fn open_owner_only(path: &Path) -> std::io::Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn open_owner_only(path: &Path) -> std::io::Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

/// Atomic write with owner-only perms: uniquely named temp file in the same directory,
/// fsync, rename into place (same rationale as the token store's `write_secure`).
fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().expect("store paths always have a parent dir");
    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;
    tmp.write_all(data)?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| e.error)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

#[cfg(unix)]
fn set_dir_private(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
}

#[cfg(not(unix))]
fn set_dir_private(_dir: &Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AppleDay, CalendarDay};

    fn date(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    #[test]
    fn empty_store_loads_as_empty_map() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        assert_eq!(store.load().unwrap(), DayMap::new());
    }

    #[test]
    fn upsert_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        let mut days = BTreeMap::new();
        days.insert(
            date("2026-07-01"),
            CalendarDay {
                meeting_hours: 4.5,
                event_count: 6,
                ..Default::default()
            },
        );
        let stats = store.upsert(days).unwrap();
        assert_eq!(
            stats,
            UpsertStats {
                added: 1,
                ..Default::default()
            }
        );
        let all = store.load().unwrap();
        assert_eq!(
            all[&date("2026-07-01")]
                .calendar
                .as_ref()
                .unwrap()
                .meeting_hours,
            4.5
        );
    }

    #[test]
    fn reimport_is_idempotent_and_replaces_only_its_own_slot() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        let cal = CalendarDay {
            meeting_hours: 2.0,
            event_count: 2,
            ..Default::default()
        };
        store
            .upsert(BTreeMap::from([(date("2026-07-01"), cal.clone())]))
            .unwrap();
        store
            .upsert(BTreeMap::from([(
                date("2026-07-01"),
                AppleDay {
                    sleep_minutes: Some(420.0),
                    ..Default::default()
                },
            )]))
            .unwrap();

        // Identical re-import: unchanged, and the other source's slot survives.
        let stats = store
            .upsert(BTreeMap::from([(date("2026-07-01"), cal.clone())]))
            .unwrap();
        assert_eq!(stats.unchanged, 1, "identical re-import must be a no-op");
        let day = &store.load().unwrap()[&date("2026-07-01")];
        assert_eq!(day.apple.as_ref().unwrap().sleep_minutes, Some(420.0));

        // Changed re-import: the calendar slot is REPLACED wholesale, apple untouched.
        let stats = store
            .upsert(BTreeMap::from([(
                date("2026-07-01"),
                CalendarDay {
                    meeting_hours: 3.0,
                    event_count: 3,
                    ..Default::default()
                },
            )]))
            .unwrap();
        assert_eq!(stats.updated, 1);
        let day = &store.load().unwrap()[&date("2026-07-01")];
        assert_eq!(day.calendar.as_ref().unwrap().meeting_hours, 3.0);
        assert_eq!(day.apple.as_ref().unwrap().sleep_minutes, Some(420.0));
    }

    #[test]
    fn a_new_slot_on_an_existing_day_counts_as_updated_not_added() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        store
            .upsert(BTreeMap::from([(
                date("2026-07-01"),
                CalendarDay::default(),
            )]))
            .unwrap();
        let stats = store
            .upsert(BTreeMap::from([(date("2026-07-01"), AppleDay::default())]))
            .unwrap();
        assert_eq!(
            stats,
            UpsertStats {
                updated: 1,
                ..Default::default()
            }
        );
    }

    #[cfg(unix)]
    #[test]
    fn store_records_and_dir_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let store_dir = dir.path().join("data");
        let store = HealthStore::with_dir(&store_dir);
        store
            .upsert(BTreeMap::from([(date("2026-07-01"), AppleDay::default())]))
            .unwrap();
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&store_dir), 0o700, "store dir must be 0700");
        assert_eq!(
            mode(&store_dir.join(STORE_FILE)),
            0o600,
            "record must be 0600"
        );
    }

    #[test]
    fn a_newer_store_version_is_a_typed_refusal_not_a_silent_wipe() {
        let dir = tempfile::tempdir().unwrap();
        let store_dir = dir.path().join("data");
        fs::create_dir_all(&store_dir).unwrap();
        fs::write(store_dir.join(STORE_FILE), r#"{"version": 99, "days": {}}"#).unwrap();
        let store = HealthStore::with_dir(&store_dir);
        let err = store.load().unwrap_err();
        assert!(
            err.to_string().contains("store version 99"),
            "must name the version conflict: {err}"
        );
    }

    #[test]
    fn concurrent_upserts_from_real_threads_lose_no_days() {
        // Real concurrency for a concurrency claim (CLAUDE.md TESTING rule 4): two
        // threads upserting DIFFERENT sources into overlapping days must both land.
        // With a no-op lock this read-merge-write races and one side's days vanish.
        let dir = tempfile::tempdir().unwrap();
        let store_dir = dir.path().join("data");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let mk = |i: u32| date(&format!("2026-07-{:02}", i + 1));

        let s1 = HealthStore::with_dir(&store_dir);
        let b1 = barrier.clone();
        let t1 = std::thread::spawn(move || {
            b1.wait();
            for i in 0..20 {
                s1.upsert(BTreeMap::from([(
                    mk(i),
                    CalendarDay {
                        event_count: i,
                        ..Default::default()
                    },
                )]))
                .unwrap();
            }
        });
        let s2 = HealthStore::with_dir(&store_dir);
        let b2 = barrier.clone();
        let t2 = std::thread::spawn(move || {
            b2.wait();
            for i in 0..20 {
                s2.upsert(BTreeMap::from([(
                    mk(i),
                    AppleDay {
                        steps: Some(f64::from(i)),
                        ..Default::default()
                    },
                )]))
                .unwrap();
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();

        let all = HealthStore::with_dir(&store_dir).load().unwrap();
        for i in 0..20 {
            let day = &all[&mk(i)];
            assert!(
                day.calendar.is_some() && day.apple.is_some(),
                "day {i}: both writers' slots must survive concurrent upserts"
            );
        }
    }

    #[test]
    fn data_dir_resolution_prefers_xdg_and_ignores_unusable_values() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |key: &str| {
                pairs
                    .iter()
                    .find(|(k, _)| *k == key)
                    .map(|(_, v)| v.to_string())
            }
        };
        #[cfg(not(windows))]
        {
            let dir =
                data_dir_from(&env(&[("XDG_DATA_HOME", "/xdg"), ("HOME", "/home/u")])).unwrap();
            assert_eq!(dir, PathBuf::from("/xdg/oura-toolkit"));
            let dir = data_dir_from(&env(&[("HOME", "/home/u")])).unwrap();
            assert_eq!(dir, PathBuf::from("/home/u/.local/share/oura-toolkit"));
            for bad in ["", "relative/path"] {
                let pairs: &[(&str, &str)] = match bad {
                    "" => &[("XDG_DATA_HOME", ""), ("HOME", "/home/u")],
                    _ => &[("XDG_DATA_HOME", "relative/path"), ("HOME", "/home/u")],
                };
                let pairs = pairs.to_vec();
                let dir = data_dir_from(&move |key: &str| {
                    pairs
                        .iter()
                        .find(|(k, _)| *k == key)
                        .map(|(_, v)| v.to_string())
                })
                .unwrap();
                assert_eq!(
                    dir,
                    PathBuf::from("/home/u/.local/share/oura-toolkit"),
                    "unusable XDG_DATA_HOME {bad:?} must fall back to HOME"
                );
            }
            assert!(matches!(
                data_dir_from(&|_| None),
                Err(HealthError::NoDataDir)
            ));
        }
        #[cfg(windows)]
        {
            let dir =
                data_dir_from(&env(&[("LOCALAPPDATA", "C:\\Users\\u\\AppData\\Local")])).unwrap();
            assert_eq!(
                dir,
                PathBuf::from("C:\\Users\\u\\AppData\\Local\\oura-toolkit\\data")
            );
        }
    }

    #[test]
    fn habit_log_is_idempotent_and_coexists_with_source_slots() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        let day = date("2026-07-01");
        store
            .upsert(BTreeMap::from([(
                day,
                CalendarDay {
                    meeting_hours: 2.0,
                    ..Default::default()
                },
            )]))
            .unwrap();

        let (name, changed) = store.log_habit(day, "Strength Training").unwrap();
        assert_eq!(name, "strength-training", "name is canonicalized");
        assert!(changed, "first log is new");
        let (_, changed) = store.log_habit(day, "strength-training").unwrap();
        assert!(!changed, "re-log is an idempotent no-op");
        store.log_habit(day, "meditate").unwrap();

        let rec = &store.load().unwrap()[&day];
        assert_eq!(rec.habits.as_ref().unwrap().done.len(), 2);
        assert_eq!(
            rec.calendar.as_ref().unwrap().meeting_hours,
            2.0,
            "habit writes must not touch source slots"
        );

        // A calendar re-import must not touch habits either (slot independence both ways).
        store
            .upsert(BTreeMap::from([(day, CalendarDay::default())]))
            .unwrap();
        let rec = &store.load().unwrap()[&day];
        assert_eq!(rec.habits.as_ref().unwrap().done.len(), 2);
    }

    #[test]
    fn habit_unlog_removes_and_leaves_no_husks() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        let day = date("2026-07-01");
        store.log_habit(day, "meditate").unwrap();

        let (_, changed) = store.unlog_habit(day, "MEDITATE").unwrap();
        assert!(changed, "normalized name matches the stored log");
        assert!(
            !store.load().unwrap().contains_key(&day),
            "an emptied record is dropped, not stored as a husk"
        );
        let (_, changed) = store.unlog_habit(day, "meditate").unwrap();
        assert!(!changed, "undoing an absent log is a no-op");
    }

    #[test]
    fn invalid_habit_names_are_typed_errors_and_never_stored() {
        let dir = tempfile::tempdir().unwrap();
        let store = HealthStore::with_dir(dir.path().join("data"));
        let err = store
            .log_habit(date("2026-07-01"), "!!!\u{202e}")
            .unwrap_err();
        assert!(
            matches!(err, HealthError::InvalidHabitName { .. }),
            "hostile name must be a typed refusal: {err}"
        );
        assert!(store.load().unwrap().is_empty(), "nothing was written");
    }

    #[test]
    fn concurrent_habit_logs_from_real_threads_lose_no_logs() {
        // Same real-concurrency requirement as the upsert test: two threads logging
        // DIFFERENT habits into the same days race read-modify-write; with a no-op
        // lock one side's logs vanish.
        let dir = tempfile::tempdir().unwrap();
        let store_dir = dir.path().join("data");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let mk = |i: u32| date(&format!("2026-07-{:02}", i + 1));

        let handles: Vec<_> = ["exercise", "meditate"]
            .into_iter()
            .map(|habit| {
                let store = HealthStore::with_dir(&store_dir);
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    for i in 0..15 {
                        store.log_habit(mk(i), habit).unwrap();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }

        let all = HealthStore::with_dir(&store_dir).load().unwrap();
        for i in 0..15 {
            let done = &all[&mk(i)].habits.as_ref().unwrap().done;
            assert!(
                done.contains("exercise") && done.contains("meditate"),
                "day {i}: both writers' logs must survive: {done:?}"
            );
        }
    }

    #[test]
    fn sync_root_paths_are_flagged_and_local_paths_are_not() {
        for synced in [
            "/home/u/Dropbox/data/oura-toolkit",
            "/Users/u/Library/Mobile Documents/oura-toolkit",
            "/Users/u/Library/Mobile Documents/com~apple~CloudDocs/oura-toolkit",
            "/home/u/OneDrive/oura-toolkit",
        ] {
            let warning = sync_root_warning(Path::new(synced));
            assert!(warning.is_some(), "{synced} must warn");
            assert!(
                warning.unwrap().contains("cloud-synced"),
                "warning names the hazard"
            );
        }
        assert_eq!(
            sync_root_warning(Path::new("/home/u/.local/share/oura-toolkit")),
            None,
            "a normal local path must not warn"
        );
    }
}
