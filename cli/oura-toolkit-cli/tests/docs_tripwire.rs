//! Docs tripwires (#45): the READMEs' ENUMERABLE claims pinned to source, so doc drift
//! fails CI instead of relying on reviewer diligence (CLAUDE.md → DOCS STAY TRUE TO THE
//! CODE, rule 4). These guard exactly the claims a test can enumerate — command list,
//! scope string, redirect URI, store paths, `just` recipe names, MCP tool names; prose
//! accuracy stays a review-gate responsibility (rules 1–3).
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
/// contribute tokens.
fn help_subcommands(help: &str) -> BTreeSet<String> {
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
        cmds.len() >= 10,
        "parsed only {} subcommands from `oura --help` — help format changed, parser broken?",
        cmds.len()
    );
    cmds
}

/// Every fenced code block plus every inline code span, one string per snippet. Command
/// and recipe tokens are only trusted inside code, never prose ("just happens" must not
/// parse as a recipe reference).
fn code_snippets(markdown: &str) -> Vec<String> {
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
    let full = help_subcommands(&oura_stdout(&["--help"]));
    let mut data = full.clone();
    for meta in ["auth", "mcp", "help"] {
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
            // the first token after `oura `.
            let Some(idx) = line.find("oura ") else {
                continue;
            };
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
