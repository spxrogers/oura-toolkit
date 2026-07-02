//! Sync guard for the crate-local spec bundle (#11): `openapi.json` in this crate is what
//! the build script reads inside a published crates.io package, so it MUST stay
//! byte-identical to the repo's vendored `spec/openapi.json`. `just spec-fetch` refreshes
//! both; this test fails CI when they drift.

use std::path::PathBuf;

#[test]
fn bundled_spec_matches_the_repo_root_spec() {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bundled = std::fs::read(crate_dir.join("openapi.json")).expect("crate-local openapi.json");

    let mut dir = crate_dir;
    let root_spec = loop {
        let candidate = dir.join("spec").join("openapi.json");
        if candidate.is_file() {
            break candidate;
        }
        assert!(dir.pop(), "repo root spec/openapi.json not found");
    };
    let root = std::fs::read(root_spec).expect("repo spec/openapi.json");

    assert!(
        bundled == root,
        "crate-local openapi.json is out of sync with spec/openapi.json — run `just spec-fetch`"
    );
}
