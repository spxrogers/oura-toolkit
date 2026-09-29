//! Binary-level enforcement of the re-consent notice (#116): spawn the REAL `oura` binary
//! with a saved login that predates a scope change, and pin the non-interactive contract
//! (docs/cli-contract.md → Scope changes):
//!
//! - the command still SUCCEEDS with its normal stdout result (the notice never blocks),
//! - exactly ONE `oura: note:` line naming the missing scopes goes to stderr,
//! - it is shown once per scope set (a second run is silent), and
//! - a current grant, or an `OURA_ACCESS_TOKEN` run, never sees it.
//!
//! A spawned child's stdin/stderr are pipes, never TTYs, so this is exactly the scripted
//! path. The interactive prompt (Y → login, n → remembered) is covered by `reauth`'s unit
//! tests through injected IO. Hermetic: the Oura host is a loopback wiremock.

use std::process::{Command, Output};

use oura_toolkit_auth::{metadata, ClientCredentials, TokenStore, Tokens};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer};

mod common;
use common::{page, sleep_doc};

const PRE_1_41_GRANT: &str = "personal daily heartrate workout tag session spo2Daily";

struct Fixture {
    server: MockServer,
    rt: tokio::runtime::Runtime,
    dir: tempfile::TempDir,
}

impl Fixture {
    /// A mock Oura serving one sleep doc, and a store holding a login with `scope`.
    fn new(scope: Option<&str>) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let server = rt.block_on(async {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/daily_sleep"))
                .respond_with(page(vec![sleep_doc("2026-06-26", 77)], None))
                .mount(&server)
                .await;
            server
        });
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path().join("oura-toolkit"));
        store
            .save_credentials(&ClientCredentials {
                client_id: "cid".into(),
                client_secret: "sec".into(),
            })
            .unwrap();
        store
            .save_tokens(&Tokens {
                access_token: "at".into(),
                refresh_token: "rt".into(),
                expires_at: 4_102_444_800, // 2100: never refreshed, so no token endpoint
                scope: scope.map(Into::into),
                token_type: None,
            })
            .unwrap();
        Fixture { server, rt, dir }
    }

    fn run(&self, extra_env: &[(&str, &str)]) -> Output {
        let _guard = self.rt.enter();
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
        cmd.env("XDG_CONFIG_HOME", self.dir.path())
            .env("HOME", self.dir.path())
            .env("LOCALAPPDATA", self.dir.path())
            .env("NO_COLOR", "1")
            .env_remove("OURA_ACCESS_TOKEN")
            .env("OURA_API_BASE_URL", self.server.uri())
            .args(["sleep", "--date", "2026-06-26"]);
        for (k, v) in extra_env {
            cmd.env(k, v);
        }
        cmd.output().expect("spawn oura")
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_stale_login_gets_one_stderr_notice_and_the_command_still_succeeds() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));

    let first = fx.run(&[]);
    let (stdout, stderr) = (text(&first.stdout), text(&first.stderr));
    assert!(
        first.status.success(),
        "the notice must never block: {stderr}"
    );
    assert!(
        stdout.contains("2026-06-26"),
        "the result still reaches stdout: {stdout}"
    );
    assert!(
        !stdout.contains("oura: note"),
        "the notice is prose — stderr only (contract → Streams): {stdout}"
    );
    assert_eq!(
        stderr.lines().count(),
        1,
        "exactly one notice line: {stderr}"
    );
    assert!(
        stderr.starts_with("oura: note:") && stderr.contains("spo2 heart_health"),
        "the notice names the missing scopes: {stderr}"
    );
    assert!(stderr.contains("oura auth login"), "and the fix: {stderr}");

    let second = fx.run(&[]);
    assert!(second.status.success());
    assert_eq!(text(&second.stderr), "", "shown once per scope set");
}

#[test]
fn a_current_login_is_never_noticed() {
    let fx = Fixture::new(Some(&metadata::default_scopes().join(" ")));
    let out = fx.run(&[]);
    assert!(out.status.success());
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn an_env_token_run_bypasses_the_store_and_the_notice() {
    // OURA_ACCESS_TOKEN never touches the store, so a stale stored grant is irrelevant.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let out = fx.run(&[("OURA_ACCESS_TOKEN", "env-tok")]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert_eq!(text(&out.stderr), "");
}
