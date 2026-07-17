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
    // generators (#27), like `mcp` is a mode and `auth` is its own surface; `api` (#19) is
    // the raw authenticated escape hatch, not a curated data command.
    for meta in ["auth", "mcp", "help", "completion", "man", "api"] {
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

/// `--date` is offered on every windowed data command (shared `RangeArgs`, #39); the README
/// and cli-contract reference it. A flag rename or a docs drop fails here — mechanizing the
/// enumerable flag claim like the `--no-browser` tripwire.
#[test]
fn date_flag_is_documented_where_it_is_offered() {
    let help = oura_stdout(&["sleep", "--help"]);
    assert!(
        help.contains("--date"),
        "`oura sleep --help` no longer lists --date:\n{help}"
    );
    let root = repo_root();
    for doc in ["README.md", "docs/cli-contract.md"] {
        assert!(
            read(&root.join(doc)).contains("--date"),
            "{doc} does not mention the --date flag (DOCS STAY TRUE TO THE CODE)"
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

// ---------------------------------------------------------------------------------------------
// Docs site (docs-site/) — the SAME enumerable-claim discipline extended to the published
// documentation website. The API reference is generated from the spec at build time and the CLI
// reference is generated from the binary (drift-checked by `just docs-gen-cli-check`), so those
// can't drift; these tests pin the HAND-WRITTEN guide/SDK pages' enumerable claims to source,
// exactly as the README tests above do, so a code change that orphans the site fails CI.
// ---------------------------------------------------------------------------------------------

/// A docs-site content page's repo-relative path.
fn docs_site_page(rel: &str) -> PathBuf {
    repo_root().join("docs-site/src/content/docs").join(rel)
}

/// The hand-written docs-site content pages. `md_only` restricts to `.md` (the `code_snippets`
/// scanner is Markdown-only; `.mdx` splash/wrapper pages are excluded from fence scans). Always
/// excluded: Astro-ignored `_`-prefixed fragments, and the GENERATED `cli/reference.md` (it is
/// drift-checked by `just docs-gen-cli-check`, not tripwired).
fn docs_site_pages(md_only: bool) -> Vec<PathBuf> {
    let base = repo_root().join("docs-site/src/content/docs");
    let mut out = Vec::new();
    let mut stack = vec![base];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("reading {dir:?}: {e}")) {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('_') {
                continue; // Astro-ignored fragment (e.g. _reference.header.md)
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !(ext == "md" || (!md_only && ext == "mdx")) {
                continue;
            }
            if path.ends_with("cli/reference.md") {
                continue; // generated + drift-checked, not tripwired
            }
            out.push(path);
        }
    }
    assert!(
        out.len() >= 10,
        "walked only {} docs-site pages — content moved or walker broken?",
        out.len()
    );
    out.sort();
    out
}

/// Every `oura <cmd>` shown in a docs-site guide is a real subcommand (one-direction, like the
/// README tripwire but for the site). Completeness stays README-anchored; this catches a guide
/// naming a command that doesn't exist.
#[test]
fn docs_site_command_references_are_real() {
    let full = help_subcommands(&oura_stdout(&["--help"]), 12);
    let mut checked = 0;
    for page in docs_site_pages(true) {
        for snippet in code_snippets(&read(&page)) {
            for line in snippet.lines() {
                for (idx, _) in line.match_indices("oura ") {
                    if idx > 0 && line.as_bytes()[idx - 1].is_ascii_alphanumeric() {
                        continue; // `npx -y oura-toolkit …` etc.
                    }
                    let token: String = line[idx + 5..]
                        .chars()
                        .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                        .collect();
                    if !token.starts_with(|c: char| c.is_ascii_lowercase()) || token == "toolkit" {
                        continue;
                    }
                    assert!(
                        full.contains(&token),
                        "{page:?} shows `oura {token}` but the binary has no such subcommand — \
                         renamed without updating the docs site? (DOCS STAY TRUE TO THE CODE)"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(
        checked >= 8,
        "suspiciously few `oura <cmd>` references across docs-site ({checked}) — tokenizer broken?"
    );
}

/// Every `oura auth <sub>` shown in a docs-site guide is a real auth subcommand.
#[test]
fn docs_site_auth_subcommand_references_are_real() {
    let mut auth = help_subcommands(&oura_stdout(&["auth", "--help"]), 5);
    auth.remove("help");
    let mut checked = 0;
    for page in docs_site_pages(true) {
        for snippet in code_snippets(&read(&page)) {
            for line in snippet.lines() {
                for (idx, _) in line.match_indices("oura auth ") {
                    if idx > 0 && line.as_bytes()[idx - 1].is_ascii_alphanumeric() {
                        continue;
                    }
                    let token: String = line[idx + 10..]
                        .chars()
                        .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                        .collect();
                    if !token.starts_with(|c: char| c.is_ascii_lowercase()) {
                        continue;
                    }
                    assert!(
                        auth.contains(&token),
                        "{page:?} shows `oura auth {token}` but the binary has no such auth \
                         subcommand (DOCS STAY TRUE TO THE CODE)"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(
        checked >= 4,
        "suspiciously few `oura auth <sub>` references across docs-site ({checked})"
    );
}

/// Any `just <recipe>` a docs-site page mentions exists in the justfile (guards a page that
/// references a renamed docs recipe). No minimum: guides mostly speak `oura`/`npx`, not `just`.
#[test]
fn docs_site_just_recipe_references_all_exist() {
    let root = repo_root();
    let mut recipes = BTreeSet::new();
    for line in read(&root.join("justfile")).lines() {
        if !line.starts_with(|c: char| c.is_ascii_alphabetic()) || line.contains(":=") {
            continue;
        }
        if line.contains(':') {
            recipes.insert(
                line.split(':')
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    assert!(recipes.len() >= 15, "justfile recipe parser broken?");
    for page in docs_site_pages(true) {
        for snippet in code_snippets(&read(&page)) {
            for (idx, _) in snippet.match_indices("just ") {
                if idx > 0 {
                    let prev = snippet.as_bytes()[idx - 1];
                    if prev.is_ascii_alphanumeric() || prev == b'-' || prev == b'_' {
                        continue; // e.g. "adjust "
                    }
                }
                let token: String = snippet[idx + 5..]
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                    .collect();
                if token.is_empty() {
                    continue;
                }
                assert!(
                    recipes.contains(&token),
                    "{page:?} references `just {token}` but the justfile has no such recipe \
                     (DOCS STAY TRUE TO THE CODE)"
                );
            }
        }
    }
}

/// Every `get_*` token on a docs-site page is a real MCP tool name (the mcp-server guide lists
/// the eight-tool table). A tool rename that skips the site fails here.
#[test]
fn docs_site_mcp_tool_names_are_real() {
    let known: BTreeSet<&str> = oura_toolkit_cli::mcp::tool_names().collect();
    let mut checked = 0;
    for page in docs_site_pages(false) {
        let text = read(&page);
        for token in text
            .split(|c: char| !(c.is_ascii_lowercase() || c == '_'))
            .filter(|t| t.starts_with("get_"))
        {
            assert!(
                known.contains(token),
                "{page:?} references unknown MCP tool {token:?} (DOCS STAY TRUE TO THE CODE)"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 8,
        "expected the eight get_* tools listed on docs-site (found {checked}) — mcp-server page changed?"
    );
}

/// The docs-site auth guide's Scopes value ⟷ the spec-derived default scope set.
#[test]
fn docs_site_scope_string_matches_the_spec_derived_default() {
    let expected = oura_toolkit_auth::metadata::default_scopes().join(" ");
    let text = read(&docs_site_page("guides/authentication.md"));
    assert!(
        text.contains(&format!("`{expected}`")),
        "docs-site authentication guide's Scopes value drifted from \
         metadata::default_scopes() — expected `{expected}`"
    );
}

/// The docs-site auth guide's Redirect URI + port ⟷ the binary's real default `--port`.
#[test]
fn docs_site_redirect_uri_matches_the_cli_default_port() {
    let help = oura_stdout(&["auth", "login", "--help"]);
    let default_port: u16 = help
        .split("[default: ")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .expect("`oura auth login --help` lost its `[default: <port>]` marker")
        .trim()
        .parse()
        .expect("default port parses as u16");
    let text = read(&docs_site_page("guides/authentication.md"));
    assert!(
        text.contains(&format!("http://localhost:{default_port}/callback")),
        "docs-site auth guide's Redirect URI drifted from the default port {default_port}"
    );
    assert!(
        text.contains(&format!("port {default_port}")),
        "docs-site auth guide no longer names the default port {default_port}"
    );
}

/// The docs-site auth guide's token-store paths ⟷ the auth crate's locked directory name.
#[test]
fn docs_site_store_paths_match_the_auth_crates_dir_name() {
    let unix = format!("~/.config/{}/", oura_toolkit_auth::APP_DIR_NAME);
    let windows = format!("%LOCALAPPDATA%\\{}\\", oura_toolkit_auth::APP_DIR_NAME);
    let text = read(&docs_site_page("guides/authentication.md"));
    for claim in [&unix, &windows] {
        assert!(
            text.contains(claim.as_str()),
            "docs-site auth guide lost the store-path claim {claim:?} \
             (source: oura_toolkit_auth::APP_DIR_NAME)"
        );
    }
}

/// Every `OURA_*` override read in api.rs is documented in the docs-site headless/CI guide.
#[test]
fn docs_site_env_overrides_documented() {
    let root = repo_root();
    let src = read(&root.join("cli/oura-toolkit-cli/src/api.rs"));
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
    let text = read(&docs_site_page("guides/headless-ci.md"));
    for name in &names {
        assert!(
            text.contains(name.as_str()),
            "docs-site headless/CI guide does not document the env override {name} read in api.rs \
             (DOCS STAY TRUE TO THE CODE)"
        );
    }
}

/// The rate-limit numbers the docs-site CLI guide states ⟷ the constants the code enforces.
#[test]
fn docs_site_rate_limit_numbers_match_the_constants() {
    let cap = format!(
        "{} seconds",
        oura_toolkit_cli::api::RATE_LIMIT_WAIT_CAP_SECS
    );
    let budget = format!(
        "{} rate-limit waits",
        oura_toolkit_cli::api::RATE_LIMIT_MAX_WAITS
    );
    let text = read(&docs_site_page("guides/cli-usage.md"));
    assert!(
        text.contains(&cap),
        "docs-site CLI guide no longer states the Retry-After cap as {cap:?} \
         (source: api::RATE_LIMIT_WAIT_CAP_SECS)"
    );
    assert!(
        text.contains(&budget),
        "docs-site CLI guide no longer states the per-invocation budget as {budget:?} \
         (source: api::RATE_LIMIT_MAX_WAITS)"
    );
}

/// The GENERATED CLI reference has a section for every real command (top-level + `oura auth`),
/// so a help-format change that breaks `just docs-gen-cli`'s enumeration — silently dropping a
/// command from the reference — fails CI instead of shipping an incomplete page.
#[test]
fn docs_site_cli_reference_covers_every_command() {
    let full = help_subcommands(&oura_stdout(&["--help"]), 12);
    let mut auth = help_subcommands(&oura_stdout(&["auth", "--help"]), 5);
    auth.remove("help");
    let reference = read(&docs_site_page("cli/reference.md"));
    for cmd in &full {
        if cmd == "help" {
            continue; // `help` is not given its own reference section
        }
        assert!(
            reference.contains(&format!("`oura {cmd}`")),
            "docs-site cli/reference.md is missing a section for `oura {cmd}` — \
             run `just docs-gen-cli` and commit (the enumeration missed a command?)"
        );
    }
    for sub in &auth {
        assert!(
            reference.contains(&format!("`oura auth {sub}`")),
            "docs-site cli/reference.md is missing a section for `oura auth {sub}` — \
             run `just docs-gen-cli` and commit"
        );
    }
}

/// The docs-site SDK pages ⟷ the languages the justfile actually generates (`generated_dirs`):
/// exactly one `sdks/<lang>.md` per generated language, and no orphan SDK page. A seventh SDK —
/// or a dropped one — fails CI until the site follows.
#[test]
fn docs_site_sdk_pages_match_generated_languages() {
    let justfile = read(&repo_root().join("justfile"));
    let line = justfile
        .lines()
        .find(|l| l.trim_start().starts_with("generated_dirs"))
        .expect("justfile lost its generated_dirs var");
    let langs: BTreeSet<String> = line
        .split_whitespace()
        .filter_map(|tok| {
            tok.trim_matches('"')
                .strip_prefix("sdks/")
                .map(str::to_string)
        })
        .filter_map(|rest| rest.split('/').next().map(str::to_string))
        .collect();
    assert!(
        langs.len() >= 6,
        "parsed only {} SDK languages from generated_dirs: {langs:?}",
        langs.len()
    );
    let sdk_dir = repo_root().join("docs-site/src/content/docs/sdks");
    for lang in &langs {
        assert!(
            sdk_dir.join(format!("{lang}.md")).is_file(),
            "docs-site is missing sdks/{lang}.md — every generated SDK language needs a docs \
             page (source: justfile generated_dirs)"
        );
    }
    for entry in std::fs::read_dir(&sdk_dir).expect("read docs-site sdks dir") {
        let path = entry.expect("entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with(".md") || name == "index.md" || name.starts_with('_') {
            continue;
        }
        let lang = name.trim_end_matches(".md");
        assert!(
            langs.contains(lang),
            "docs-site sdks/{name} has no matching generated language in the justfile \
             generated_dirs — orphan SDK page?"
        );
    }
}

/// The documented npm install command ⟷ the real package.json names (#96). The published-SDK
/// claim is enumerable: both TS packages are named per the NAMING lock, and every doc that
/// tells users to install them uses exactly those names. Renaming a package (or documenting a
/// package that doesn't exist) fails CI here, not on npmjs.com.
#[test]
fn documented_npm_sdk_install_matches_package_names() {
    let root = repo_root();
    let mut names = Vec::new();
    for (dir, expected) in [
        ("sdks/typescript/api", "@oura-toolkit/api"),
        ("sdks/typescript/auth", "@oura-toolkit/auth"),
    ] {
        let manifest = read(&root.join(dir).join("package.json"));
        let parsed: serde_json::Value =
            serde_json::from_str(&manifest).unwrap_or_else(|e| panic!("{dir}/package.json: {e}"));
        let name = parsed["name"].as_str().expect("package.json has a name");
        assert_eq!(
            name, expected,
            "{dir}/package.json is named {name}, not the NAMING-locked {expected}"
        );
        names.push(name.to_string());
    }
    // The single install line every doc claim must carry verbatim (api first, like the docs).
    let install = format!("npm install {}", names.join(" "));
    for doc in [
        root.join("README.md"),
        root.join("sdks/typescript/auth/README.md"),
        docs_site_page("sdks/typescript.md"),
    ] {
        assert!(
            read(&doc).contains(&install),
            "{doc:?} does not carry the npm install command `{install}` — package renamed or \
             install docs drifted (DOCS STAY TRUE TO THE CODE)"
        );
    }
}

/// The documented pip install command ⟷ the real pyproject.toml dist name (#96, the PyPI
/// sibling of the npm tripwire above). The single Python dist is NAMING-locked to
/// `oura-toolkit`; every doc that tells users to install it must use exactly that name.
#[test]
fn documented_pypi_install_matches_dist_name() {
    let root = repo_root();
    let pyproject = read(&root.join("sdks/python/pyproject.toml"));
    let name = pyproject
        .lines()
        .find_map(|l| {
            l.strip_prefix("name = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .expect("sdks/python/pyproject.toml lost its [project] name line");
    assert_eq!(
        name, "oura-toolkit",
        "pyproject.toml dist is named {name}, not the NAMING-locked oura-toolkit"
    );
    let install = format!("pip install {name}");
    for doc in [
        root.join("README.md"),
        root.join("sdks/python/README.md"),
        docs_site_page("sdks/python.md"),
    ] {
        assert!(
            read(&doc).contains(&install),
            "{doc:?} does not carry the install command `{install}` — dist renamed or install \
             docs drifted (DOCS STAY TRUE TO THE CODE)"
        );
    }
}

/// The documented `go get` path ⟷ the real go.mod module line (#96, the Go sibling of the npm
/// and PyPI tripwires above). The nested module's path is NAMING-locked; a moved/renamed
/// module (or docs pointing at a path that doesn't exist) fails CI here, not at `go get` time.
#[test]
fn documented_go_get_matches_module_path() {
    let root = repo_root();
    let gomod = read(&root.join("sdks/go/go.mod"));
    let module = gomod
        .lines()
        .find_map(|l| l.strip_prefix("module "))
        .expect("sdks/go/go.mod lost its module line")
        .trim();
    assert_eq!(
        module, "github.com/spxrogers/oura-toolkit/sdks/go",
        "go.mod module path is {module}, not the NAMING-locked repo-subdirectory path"
    );
    let install = format!("go get {module}");
    for doc in [root.join("README.md"), docs_site_page("sdks/go.md")] {
        assert!(
            read(&doc).contains(&install),
            "{doc:?} does not carry the install command `{install}` — module moved or install \
             docs drifted (DOCS STAY TRUE TO THE CODE)"
        );
    }
    // The sub-tag choreography that makes versions resolvable: the publish workflow must
    // carry a job invoking the recipe (a deleted job would silently stop tagging). Match the
    // step's `run:` line, NOT the bare recipe name — the header comment also mentions the
    // recipe, and a comment must not satisfy this (break-verified by deleting the job).
    let workflow = read(&root.join(".github/workflows/publish-sdks.yml"));
    assert!(
        workflow.contains("run: just sdk-publish-go"),
        "publish-sdks.yml no longer runs `just sdk-publish-go` — Go releases would silently \
         stop being tagged (sdks/go/vX.Y.Z)"
    );
}

/// The documented `dotnet add package` commands ⟷ the real csproj PackageIds (#96, the NuGet
/// sibling of the npm/PyPI/Go tripwires above). Both C# packages are NAMING-locked; every doc
/// that tells users to install them uses exactly those ids, and the publish workflow must keep
/// running the recipe that ships them.
#[test]
fn documented_nuget_install_matches_package_ids() {
    let root = repo_root();
    let mut ids = Vec::new();
    for (csproj, expected) in [
        (
            "sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj",
            "OuraToolkit.Api",
        ),
        (
            "sdks/csharp/auth/src/OuraToolkit.Auth/OuraToolkit.Auth.csproj",
            "OuraToolkit.Auth",
        ),
    ] {
        let content = read(&root.join(csproj));
        let id = content
            .lines()
            .find_map(|l| {
                l.trim()
                    .strip_prefix("<PackageId>")
                    .and_then(|rest| rest.strip_suffix("</PackageId>"))
            })
            .unwrap_or_else(|| panic!("{csproj} lost its <PackageId>"));
        assert_eq!(
            id, expected,
            "{csproj} PackageId is {id}, not the NAMING-locked {expected}"
        );
        ids.push(id.to_string());
    }
    for doc in [
        root.join("README.md"),
        root.join("sdks/csharp/auth/README.md"),
        docs_site_page("sdks/csharp.md"),
    ] {
        let content = read(&doc);
        for id in &ids {
            let install = format!("dotnet add package {id}");
            assert!(
                content.contains(&install),
                "{doc:?} does not carry `{install}` — package renamed or install docs drifted \
                 (DOCS STAY TRUE TO THE CODE)"
            );
        }
    }
    // The publish workflow must keep shipping them (match the step's `run:` line, not the
    // recipe name — header comments also mention it; see the Go tripwire above).
    let workflow = read(&root.join(".github/workflows/publish-sdks.yml"));
    assert!(
        workflow.contains("run: just sdk-publish-nuget"),
        "publish-sdks.yml no longer runs `just sdk-publish-nuget` — NuGet releases would \
         silently stop publishing"
    );
}

/// The documented Maven coordinates ⟷ the real pom groupId/artifactIds (#96, the final
/// sibling of the npm/PyPI/Go/NuGet tripwires above). Both artifacts are NAMING-locked to
/// `com.ouratoolkit:{api,auth}`; the install docs must carry exactly those coordinates, and
/// the publish workflow must keep running the recipe that ships them.
#[test]
fn documented_maven_coordinates_match_the_poms() {
    let root = repo_root();
    for (pom, artifact) in [
        ("sdks/java/api/pom.xml", "api"),
        ("sdks/java/auth/pom.xml", "auth"),
    ] {
        let content = read(&root.join(pom));
        assert!(
            content.contains("<groupId>com.ouratoolkit</groupId>"),
            "{pom} lost the NAMING-locked groupId com.ouratoolkit"
        );
        assert!(
            content.contains(&format!("<artifactId>{artifact}</artifactId>")),
            "{pom} lost the NAMING-locked artifactId {artifact}"
        );
    }
    // The java SDK page's install snippet: a <dependency> block per artifact, exact ids.
    let java_page = read(&docs_site_page("sdks/java.md"));
    for artifact in ["api", "auth"] {
        assert!(
            java_page.contains(&format!("<artifactId>{artifact}</artifactId>")),
            "docs-site sdks/java.md install snippet lost the {artifact} dependency \
             (DOCS STAY TRUE TO THE CODE)"
        );
    }
    assert!(
        java_page.contains("<groupId>com.ouratoolkit</groupId>"),
        "docs-site sdks/java.md install snippet lost the com.ouratoolkit groupId"
    );
    for doc in [root.join("README.md"), docs_site_page("sdks/index.md")] {
        assert!(
            read(&doc).contains("com.ouratoolkit:api"),
            "{doc:?} no longer names the com.ouratoolkit:api coordinate"
        );
    }
    let workflow = read(&root.join(".github/workflows/publish-sdks.yml"));
    assert!(
        workflow.contains("run: just sdk-publish-maven"),
        "publish-sdks.yml no longer runs `just sdk-publish-maven` — Maven Central releases \
         would silently stop publishing"
    );
}
