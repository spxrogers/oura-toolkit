//! The output layer (#17): every data command renders through here — no `println!` in
//! command code.
//!
//! Defaults are human-first, gh-style: a readable table when stdout is a TTY, stable
//! machine-parseable lines (tab-separated) when piped, and JSON only when explicitly
//! requested with `--json`. Color is used only when writing to a TTY and is disabled by
//! `NO_COLOR` (any non-empty value, per no-color.org) or `--no-color`.
//!
//! Deliberately dependency-free: TTY detection is std (`IsTerminal`), tables are computed
//! column widths, and styling is plain SGR codes behind a single gate. `--jq`-style
//! filtering, `--template`, and pager integration are the remainder of #17.

// The rendering surface is consumed by the data commands (#9); until those land it is
// exercised by this module's tests and the RenderOptions resolution in main.
#![allow(dead_code)]

use std::io::IsTerminal;

/// How a command's result should be rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Human-readable aligned table (TTY default).
    Table,
    /// Stable tab-separated lines for scripts (piped default).
    Plain,
    /// Pretty-printed JSON (explicit `--json` only — never a default).
    Json,
}

impl Format {
    /// Resolve the output format from the `--json` flag and the stdout TTY state.
    pub fn resolve(json_flag: bool, stdout_is_tty: bool) -> Self {
        if json_flag {
            Format::Json
        } else if stdout_is_tty {
            Format::Table
        } else {
            Format::Plain
        }
    }

    /// [`Self::resolve`] against the real stdout.
    pub fn detect(json_flag: bool) -> Self {
        Self::resolve(json_flag, std::io::stdout().is_terminal())
    }
}

/// Whether styled (colored/bold) output should be emitted.
///
/// True only when writing to a TTY, `NO_COLOR` is absent/empty (no-color.org), and the
/// user didn't pass `--no-color`.
pub fn resolve_color(no_color_flag: bool, stdout_is_tty: bool, no_color_env: Option<&str>) -> bool {
    stdout_is_tty && !no_color_flag && no_color_env.is_none_or(str::is_empty)
}

/// [`resolve_color`] against the real environment.
pub fn detect_color(no_color_flag: bool) -> bool {
    resolve_color(
        no_color_flag,
        std::io::stdout().is_terminal(),
        std::env::var("NO_COLOR").ok().as_deref(),
    )
}

/// Minimal SGR styling, gated on the resolved color decision.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    enabled: bool,
}

impl Style {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Bold — used for table headers.
    pub fn bold(&self, s: &str) -> String {
        self.wrap(s, "1")
    }

    /// Dim — used for secondary detail.
    pub fn dim(&self, s: &str) -> String {
        self.wrap(s, "2")
    }

    fn wrap(&self, s: &str, code: &str) -> String {
        if self.enabled {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
}

/// A curated result table: headers plus rows. Rendering depends on [`Format`]:
/// `Table` → padded aligned columns (headers bold when styled); `Plain` → one
/// tab-separated line per row, no header (stable for `cut`/`awk`).
#[derive(Debug, Default)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new<S: Into<String>>(headers: impl IntoIterator<Item = S>) -> Self {
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: Vec::new(),
        }
    }

    pub fn row<S: Into<String>>(&mut self, cells: impl IntoIterator<Item = S>) -> &mut Self {
        let cells: Vec<String> = cells.into_iter().map(Into::into).collect();
        debug_assert_eq!(
            cells.len(),
            self.headers.len(),
            "row arity must match headers"
        );
        self.rows.push(cells);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Render for the given format. `Json` is intentionally NOT handled here — JSON output
    /// must come from the command's actual data model (serde), never re-encoded table cells.
    pub fn render(&self, format: Format, style: Style) -> String {
        match format {
            Format::Table => self.render_aligned(style),
            Format::Plain => self.render_plain(),
            Format::Json => unreachable!("JSON renders from the data model, not table cells"),
        }
    }

    fn render_aligned(&self, style: Style) -> String {
        let mut widths: Vec<usize> = self.headers.iter().map(String::len).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = widths[i].max(cell.len());
            }
        }
        let mut out = String::new();
        let header_line = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ");
        out.push_str(&style.bold(header_line.trim_end()));
        out.push('\n');
        for row in &self.rows {
            let line = row
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{:<width$}", c, width = widths[i]))
                .collect::<Vec<_>>()
                .join("  ");
            out.push_str(line.trim_end());
            out.push('\n');
        }
        out
    }

    fn render_plain(&self) -> String {
        let mut out = String::new();
        for row in &self.rows {
            out.push_str(&row.join("\t"));
            out.push('\n');
        }
        out
    }
}

/// Serialize a command's data model as pretty JSON (the `--json` path).
pub fn to_json<T: serde::Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}

/// The per-invocation rendering decision, resolved once from flags + environment and
/// handed to every data command (#9).
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub format: Format,
    pub style: Style,
}

impl RenderOptions {
    pub fn from_flags(json: bool, no_color: bool) -> Self {
        Self {
            format: Format::detect(json),
            style: Style::new(detect_color(no_color)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_resolution_is_human_first() {
        assert_eq!(Format::resolve(false, true), Format::Table, "TTY → table");
        assert_eq!(
            Format::resolve(false, false),
            Format::Plain,
            "piped → plain"
        );
        assert_eq!(
            Format::resolve(true, true),
            Format::Json,
            "--json wins on TTY"
        );
        assert_eq!(
            Format::resolve(true, false),
            Format::Json,
            "--json wins piped"
        );
    }

    #[test]
    fn color_requires_tty_and_no_optouts() {
        assert!(resolve_color(false, true, None));
        assert!(!resolve_color(false, false, None), "piped → no color");
        assert!(!resolve_color(true, true, None), "--no-color wins");
        assert!(!resolve_color(false, true, Some("1")), "NO_COLOR wins");
        assert!(
            resolve_color(false, true, Some("")),
            "empty NO_COLOR is treated as unset per no-color.org"
        );
    }

    #[test]
    fn aligned_table_pads_columns_and_bolds_header() {
        let mut t = Table::new(["DAY", "SCORE"]);
        t.row(["2026-07-01", "88"]).row(["2026-07-02", "9"]);
        let plain_style = Style::new(false);
        let rendered = t.render(Format::Table, plain_style);
        assert_eq!(
            rendered,
            "DAY         SCORE\n2026-07-01  88\n2026-07-02  9\n"
        );

        let styled = t.render(Format::Table, Style::new(true));
        assert!(styled.starts_with("\x1b[1mDAY         SCORE\x1b[0m\n"));
        assert!(!styled.contains("88\x1b"), "data cells are not styled");
    }

    #[test]
    fn plain_output_is_tab_separated_with_no_header() {
        let mut t = Table::new(["DAY", "SCORE"]);
        t.row(["2026-07-01", "88"]);
        assert_eq!(
            t.render(Format::Plain, Style::new(false)),
            "2026-07-01\t88\n"
        );
    }

    #[test]
    fn styling_is_a_no_op_when_disabled() {
        let s = Style::new(false);
        assert_eq!(s.bold("x"), "x");
        assert_eq!(s.dim("x"), "x");
        let on = Style::new(true);
        assert_eq!(on.bold("x"), "\x1b[1mx\x1b[0m");
    }

    #[test]
    fn json_helper_serializes_data_models() {
        #[derive(serde::Serialize)]
        struct Day {
            day: &'static str,
            score: u8,
        }
        let json = to_json(&vec![Day {
            day: "2026-07-01",
            score: 88,
        }])
        .unwrap();
        assert!(json.contains("\"score\": 88"));
    }
}
