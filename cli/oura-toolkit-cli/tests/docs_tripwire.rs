//! Docs tripwires (#45): the READMEs' ENUMERABLE claims pinned to source, so doc drift
//! fails CI instead of relying on reviewer diligence (CLAUDE.md → DOCS STAY TRUE TO THE
//! CODE, rule 4). These guard exactly the claims a test can enumerate — command list,
//! the `oura auth` subcommand list (#63), scope string, redirect URI, store paths,
//! `just` recipe names, MCP tool names; prose accuracy stays a review-gate
//! responsibility (rules 1–3).
//!
//! Monorepo-only by nature (they read repo-root files, same walk-up as bundled_spec.rs);
//! `just publish-check` only BUILDS the packaged crate, so these never run outside the
//! repo.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Repo root: the nearest ancestor holding both the justfile and the README.
fn repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("justfile").is_file() && dir.join("README.md").is_file() {
            return dir;
        }
        assert!(
            dir.pop(),
            "repo root (justfile + README.md) not found above CARGO_MANIFEST_DIR"
        );
    }
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}

/// The `oura` binary's stdout for `args`, isolated like cli_contract.rs does it.
fn oura_stdout(args: &[&str]) -> String {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = assert_cmd::Command::cargo_bin("oura")
        .expect("oura binary")
        .env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .args(args)
        .output()
        .expect("spawn oura");
    assert!(
        out.status.success(),
        "oura {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf8 stdout")
}

/// Subcommand names parsed from clap's `Commands:` help section. Only exactly-two-space
/// indented lines count — wrapped description continuations indent deeper and must not
/// contribute tokens. `min_expected` is the parser's own tripwire: fewer parsed commands
/// than the surface is known to have means the help format changed, not that commands
/// vanished.
fn help_subcommands(help: &str, min_expected: usize) -> BTreeSet<String> {
    let mut cmds = BTreeSet::new();
    let mut in_commands = false;
    for line in help.lines() {
        if line.trim() == "Commands:" {
            in_commands = true;
            continue;
        }
        if in_commands {
            let Some(rest) = line.strip_prefix("  ") else {
                break; // blank line or next section header ends the list
            };
            if rest.starts_with(' ') {
                continue; // wrapped description continuation
            }
            cmds.insert(rest.split_whitespace().next().unwrap().to_string());
        }
    }
    assert!(
        cmds.len() >= min_expected,
        "parsed only {} subcommands from the help text (expected >= {min_expected}) — \
         help format changed, parser broken?",
        cmds.len()
    );
    cmds
}

/// Every fenced code block plus every inline code span, one string per snippet. Command
/// and recipe tokens are only trusted inside code, never prose ("just happens" must not
/// parse as a recipe reference).
fn code_snippets(markdown: &str) -> Vec<String> {
    // Tilde fences would silently read as prose (their contents skipped, a false
    // negative) — fail loudly instead of scanning blind.
    assert!(
        !markdown.contains("~~~"),
        "doc uses a ~~~ fence, which this scanner does not parse — use ``` fences \
         (or teach code_snippets about tildes)"
    );
    let mut snippets = Vec::new();
    let mut fence: Option<String> = None;
    for line in markdown.lines() {
        if line
            .trim_start()
            .trim_start_matches('>')
            .trim_start()
            .starts_with("```")
        {
            match fence.take() {
                Some(block) => snippets.push(block),
                None => fence = Some(String::new()),
            }
            continue;
        }
        if let Some(block) = fence.as_mut() {
            block.push_str(line);
            block.push('\n');
        } else {
            // Silent-under-scan guards (#63's review): an indented command block or a
            // double-backtick span would read as prose (contents skipped, a false
            // negative) — fail loudly instead of scanning blind, like the ~~~ guard.
            let trimmed = line.trim_start();
            assert!(
                !(line.starts_with("    ")
                    && (trimmed.starts_with("oura ") || trimmed.starts_with("just "))),
                "doc uses a 4-space-indented command block, which this scanner does not \
                 parse — use ``` fences: {line:?}"
            );
            let bytes = line.as_bytes();
            for (i, _) in line.match_indices("``") {
                let part_of_triple =
                    (i > 0 && bytes[i - 1] == b'`') || bytes.get(i + 2) == Some(&b'`');
                assert!(
                    part_of_triple,
                    "doc uses a ``double-backtick`` span, which this scanner does not \
                     parse — use single-backtick spans: {line:?}"
                );
            }
            for (i, piece) in line.split('`').enumerate() {
                if i % 2 == 1 {
                    snippets.push(piece.to_string());
                }
            }
        }
    }
    assert!(
        fence.is_none(),
        "unterminated code fence — markdown structure changed?"
    );
    snippets
}

/// README command list ⟷ the binary's real subcommands, both directions: every `oura
/// <cmd>` the README shows must exist, and every data subcommand the binary grows must
/// be documented. Adding a ninth data command without touching the README fails here.
#[test]
fn readme_command_list_matches_the_binary() {
    let full = help_subcommands(&oura_stdout(&["--help"]), 12);
    let mut data = full.clone();
    // Utility/meta commands (not data over your Oura account) are stripped so only the data
    // commands are held to the README's data-command list. `completion`/`man` are code
    // generators (#27), like `mcp` is a mode and `auth` is its own surface.
    for meta in ["auth", "mcp", "help", "completion", "man"] {
        assert!(
            data.remove(meta),
            "`oura --help` no longer lists the {meta:?} subcommand — parser or CLI broken?"
        );
    }

    let readme = read(&repo_root().join("README.md"));
    let mut documented = BTreeSet::new();
    for snippet in code_snippets(&readme) {
        for line in snippet.lines() {
            // `$ oura sleep`, `oura readiness --json | jq …`, `oura auth setup` — take
            // the token after EVERY `oura ` occurrence (a compound line like
            // `oura a && oura b` must have both commands checked).
            for (idx, _) in line.match_indices("oura ") {
                if idx > 0 && line.as_bytes()[idx - 1].is_ascii_alphanumeric() {
                    continue; // `npx -y oura-toolkit …` etc. — not the binary name
                }
                let token: String = line[idx + 5..]
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                    .collect();
                if !token.starts_with(|c: char| c.is_ascii_lowercase()) || token == "toolkit" {
                    continue; // bare `oura`, a flag/`--` separator, or an `oura-toolkit` remnant
                }
                assert!(
                    full.contains(&token),
                    "README shows `oura {token}` but the binary has no such subcommand — \
                     renamed without updating the docs? (DOCS STAY TRUE TO THE CODE)"
                );
                documented.insert(token);
            }
        }
    }

    let undocumented: Vec<&String> = data.iter().filter(|c| !documented.contains(*c)).collect();
    assert!(
        undocumented.is_empty(),
        "data subcommands missing from the README's command list: {undocumented:?} — \
         a new command must land with its docs in the same PR; also recheck the \
         'Eight read commands' heading (DOCS STAY TRUE TO THE CODE)"
    );
    assert!(
        documented.len() >= 8,
        "suspiciously few `oura <cmd>` references in the README ({}) — tokenizer broken?",
        documented.len()
    );
}

/// The documented `oura auth <sub>` surface ⟷ the binary's real auth subcommands, both
/// directions (#63). The top-level tripwire above tokenizes only the word after `oura `,
/// so `oura auth status` contributed just `auth` — a doc claiming a nonexistent auth
/// subcommand (or an auth subcommand landing undocumented) slipped through. This closes
/// that: every `oura auth <sub>` shown in README.md or docs/cli-contract.md must exist,
/// and every auth subcommand the binary has must appear in the README. (Completeness is
/// deliberately README-only: the README is the canonical front-door command list, so a
/// new auth command must land there; cli-contract is held to existence only.)
#[test]
fn documented_auth_subcommands_match_the_binary() {
    let mut auth = help_subcommands(&oura_stdout(&["auth", "--help"]), 5);
    assert!(
        auth.remove("help"),
        "`oura auth --help` no longer lists the help subcommand — parser or CLI broken?"
    );

    let root = repo_root();
    let mut documented_in_readme = BTreeSet::new();
    let mut checked = 0;
    for doc in ["README.md", "docs/cli-contract.md"] {
        let text = read(&root.join(doc));
        for snippet in code_snippets(&text) {
            for line in snippet.lines() {
                // `oura auth setup`, `$ oura auth logout --all`, compound lines — take
                // the token after EVERY `oura auth ` occurrence.
                for (idx, _) in line.match_indices("oura auth ") {
                    if idx > 0 && line.as_bytes()[idx - 1].is_ascii_alphanumeric() {
                        continue; // e.g. `npx -y oura-toolkit auth …` never matches; guard anyway
                    }
                    let token: String = line[idx + 10..]
                        .chars()
                        .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                        .collect();
                    if !token.starts_with(|c: char| c.is_ascii_lowercase()) {
                        continue; // bare `oura auth` or a flag follows
                    }
                    assert!(
                        auth.contains(&token),
                        "{doc} shows `oura auth {token}` but the binary has no such auth \
                         subcommand — renamed without updating the docs? (DOCS STAY TRUE \
                         TO THE CODE)"
                    );
                    checked += 1;
                    if doc == "README.md" {
                        documented_in_readme.insert(token);
                    }
                }
            }
        }
    }

    let undocumented: Vec<&String> = auth
        .iter()
        .filter(|c| !documented_in_readme.contains(*c))
        .collect();
    assert!(
        undocumented.is_empty(),
        "auth subcommands missing from the README: {undocumented:?} — a new auth command \
         must land with its docs in the same PR (DOCS STAY TRUE TO THE CODE)"
    );
    assert!(
        checked >= 6,
        "suspiciously few `oura auth <sub>` references across the docs ({checked}) — \
         tokenizer broken?"
    );
}

/// README registration walkthrough's Scopes value ⟷ the spec-derived default scope set
/// (the exact claim PR #44's review loop caught drifting).
#[test]
fn readme_scope_string_matches_the_spec_derived_default() {
    let expected = oura_toolkit_auth::metadata::default_scopes().join(" ");
    let readme = read(&repo_root().join("README.md"));
    let line = readme
        .lines()
        .find(|l| l.contains("**Scopes**:"))
        .expect("README registration walkthrough lost its **Scopes**: line");
    assert!(
        line.contains(&format!("`{expected}`")),
        "README's registration Scopes value drifted from metadata::default_scopes() — \
         expected `{expected}`, walkthrough line: {line}"
    );
}

/// README's Redirect URI ⟷ the binary's real default `--port`. The URI *shape* is pinned
/// by auth.rs's own unit test; this chains the README claim to the clap default, so a
/// default-port change orphaning the docs fails CI.
#[test]
fn readme_redirect_uri_matches_the_cli_default_port() {
    let help = oura_stdout(&["auth", "login", "--help"]);
    let default_port: u16 = help
        .split("[default: ")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .expect("`oura auth login --help` lost its `[default: <port>]` marker")
        .trim()
        .parse()
        .expect("default port parses as u16");
    let expected = format!("http://localhost:{default_port}/callback");
    let readme = read(&repo_root().join("README.md"));
    assert!(
        readme.contains(&expected),
        "README's Redirect URI drifted from the binary's default port {default_port} — \
         expected {expected} in the registration walkthrough"
    );
    assert!(
        readme.contains(&format!("port {default_port}")),
        "README prose no longer names the default port {default_port} — \
         the login walkthrough and the port-conflict tip both claim it"
    );
}

/// Both READMEs' token-store path claims ⟷ the auth crate's locked directory name.
#[test]
fn docs_store_paths_match_the_auth_crates_dir_name() {
    let unix = format!("~/.config/{}/", oura_toolkit_auth::APP_DIR_NAME);
    let windows = format!("%LOCALAPPDATA%\\{}\\", oura_toolkit_auth::APP_DIR_NAME);
    let root = repo_root();
    for doc in ["README.md", "plugins/oura-toolkit/README.md"] {
        let text = read(&root.join(doc));
        for claim in [&unix, &windows] {
            assert!(
                text.contains(claim.as_str()),
                "{doc} lost the store-path claim {claim:?} — path renamed without \
                 updating the docs? (source: oura_toolkit_auth::APP_DIR_NAME)"
            );
        }
    }
}

/// Every `just <recipe>` the docs mention exists in the justfile — a recipe rename that
/// orphans the README or CONTRIBUTING fails CI.
#[test]
fn documented_just_recipes_all_exist() {
    let root = repo_root();

    // Recipe headers sit at column 0: `name:` / `name dep…:`. Variable assignments use
    // `:=` and attributes/comments/bodies never start with a letter at column 0.
    let mut recipes = BTreeSet::new();
    for line in read(&root.join("justfile")).lines() {
        if !line.starts_with(|c: char| c.is_ascii_alphabetic()) || line.contains(":=") {
            continue;
        }
        if let Some(header) = line.split(':').next() {
            if line.contains(':') {
                recipes.insert(header.split_whitespace().next().unwrap().to_string());
            }
        }
    }
    assert!(
        recipes.len() >= 15,
        "parsed only {} justfile recipes — header parser broken?",
        recipes.len()
    );

    let mut checked = 0;
    for doc in ["README.md", "CONTRIBUTING.md"] {
        let text = read(&root.join(doc));
        for snippet in code_snippets(&text) {
            for (idx, _) in snippet.match_indices("just ") {
                if idx > 0 {
                    let prev = snippet.as_bytes()[idx - 1];
                    if prev.is_ascii_alphanumeric() || prev == b'-' || prev == b'_' {
                        continue; // e.g. "adjust ", never a recipe reference
                    }
                }
                let token: String = snippet[idx + 5..]
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                    .collect();
                if token.is_empty() {
                    continue; // bare `just` (the list-recipes default)
                }
                assert!(
                    recipes.contains(&token),
                    "{doc} references `just {token}` but the justfile has no such recipe — \
                     renamed without updating the docs? (DOCS STAY TRUE TO THE CODE)"
                );
                checked += 1;
            }
        }
    }
    assert!(
        checked >= 8,
        "suspiciously few `just <recipe>` references across the docs ({checked}) — \
         tokenizer broken?"
    );
}

/// The rate-limit numbers the docs state ⟷ the constants the code enforces (#28). Both
/// docs write the wait cap and the per-invocation budget as literals; a constant bump
/// that skips the prose must fail CI, not wait for a reader to notice.
#[test]
fn documented_rate_limit_numbers_match_the_constants() {
    let cap = format!(
        "{} seconds",
        oura_toolkit_cli::api::RATE_LIMIT_WAIT_CAP_SECS
    );
    let budget = format!(
        "{} rate-limit waits",
        oura_toolkit_cli::api::RATE_LIMIT_MAX_WAITS
    );
    let root = repo_root();
    for doc in ["README.md", "docs/cli-contract.md"] {
        let text = read(&root.join(doc));
        assert!(
            text.contains(&cap),
            "{doc} no longer states the Retry-After cap as {cap:?} — constant bumped              without the docs, or prose rephrased? (source: api::RATE_LIMIT_WAIT_CAP_SECS)"
        );
        assert!(
            text.contains(&budget),
            "{doc} no longer states the per-invocation budget as {budget:?} — constant              bumped without the docs? (source: api::RATE_LIMIT_MAX_WAITS)"
        );
    }
}

/// Every `get_*` token in the README is a real MCP tool name, and the server still has
/// exactly the eight the README's "eight curated, described tools" claim counts.
#[test]
fn readme_mcp_tool_names_are_real() {
    let known: BTreeSet<&str> = oura_toolkit_cli::mcp::tool_names().collect();
    assert_eq!(
        known.len(),
        8,
        "MCP tool count changed — the README's 'eight curated, described tools' claim \
         (and the plugin README's 'eight read-only tools') need review"
    );
    let readme = read(&repo_root().join("README.md"));
    let mut checked = 0;
    for token in readme
        .split(|c: char| !(c.is_ascii_lowercase() || c == '_'))
        .filter(|t| t.starts_with("get_"))
    {
        assert!(
            known.contains(token),
            "README references unknown MCP tool {token:?} — renamed without updating \
             the docs? (DOCS STAY TRUE TO THE CODE)"
        );
        checked += 1;
    }
    assert!(
        checked >= 2,
        "suspiciously few get_* references in the README ({checked}) — tokenizer broken?"
    );
}

/// The completion shells the docs advertise ⟷ the value-enum the binary actually accepts (#27).
/// clap derives the accepted set from `clap_complete::Shell`; if it grows or shrinks, the README
/// and cli-contract must move with it (DOCS STAY TRUE TO THE CODE, rule 4). Parsed from the
/// binary's own `[possible values: …]` so the docs are pinned to source, not a hand-copied list.
#[test]
fn documented_completion_shells_match_the_binary() {
    let help = oura_stdout(&["completion", "--help"]);
    let list = help
        .split("[possible values:")
        .nth(1)
        .expect(
            "`oura completion --help` lost its `[possible values: …]` list — clap format changed?",
        )
        .split(']')
        .next()
        .unwrap();
    let shells: BTreeSet<String> = list
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    assert!(
        shells.len() >= 4,
        "parsed only {} completion shells from the binary ({shells:?}) — help format changed?",
        shells.len()
    );

    let root = repo_root();
    for doc in ["README.md", "docs/cli-contract.md"] {
        let text = read(&root.join(doc));
        for shell in &shells {
            assert!(
                text.contains(shell.as_str()),
                "{doc} does not mention the completion shell {shell:?} that `oura completion` \
                 accepts — a shell landed undocumented (DOCS STAY TRUE TO THE CODE)"
            );
        }
    }
}

/// The #20 headless env overrides (`OURA_ACCESS_TOKEN`, `OURA_API_BASE_URL`) must be
/// documented wherever they are read. Enumerated from the source (every `OURA_*` name in
/// `api.rs`) so a new override that ships undocumented — or a rename that orphans the docs —
/// fails CI, not review.
#[test]
fn documented_env_overrides_match_the_source() {
    let root = repo_root();
    let src = read(&root.join("cli/oura-toolkit-cli/src/api.rs"));
    // Pull every `OURA_<NAME>` token out of the source (string literals + the doc comments
    // beside them). A token is the run of word chars following `OURA_`.
    let names: BTreeSet<String> = src
        .match_indices("OURA_")
        .map(|(i, _)| {
            let rest = &src[i..];
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            rest[..end].to_string()
        })
        .collect();
    assert!(
        names.contains("OURA_ACCESS_TOKEN") && names.contains("OURA_API_BASE_URL"),
        "expected the #20 env overrides among api.rs's OURA_* names: {names:?}"
    );
    let readme = read(&root.join("README.md"));
    let contract = read(&root.join("docs/cli-contract.md"));
    for name in &names {
        assert!(
            readme.contains(name.as_str()),
            "README.md does not document the env override {name} read in api.rs \
             (DOCS STAY TRUE TO THE CODE)"
        );
        assert!(
            contract.contains(name.as_str()),
            "docs/cli-contract.md does not document the env override {name} read in api.rs"
        );
    }
}

/// `--no-browser` is offered on `oura auth setup` and `oura auth login` (#20); the README and
/// cli-contract reference it. A flag rename or a docs drop fails here.
#[test]
fn no_browser_flag_is_documented_where_it_is_offered() {
    for sub in ["setup", "login"] {
        let help = oura_stdout(&["auth", sub, "--help"]);
        assert!(
            help.contains("--no-browser"),
            "`oura auth {sub} --help` no longer lists --no-browser:\n{help}"
        );
    }
    let root = repo_root();
    for doc in ["README.md", "docs/cli-contract.md"] {
        assert!(
            read(&root.join(doc)).contains("--no-browser"),
            "{doc} does not mention the --no-browser flag (DOCS STAY TRUE TO THE CODE)"
        );
    }
}
