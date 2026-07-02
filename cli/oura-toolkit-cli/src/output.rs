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
        // Cells are sanitized on entry: API-sourced strings (tags, notes) must never carry
        // terminal escape bytes to a TTY or forge rows/columns in the tab-separated plain
        // format that scripts parse.
        let cells: Vec<String> = cells.into_iter().map(|c| sanitize(&c.into())).collect();
        // A real assert, not debug_assert: a mis-sized row in release would otherwise
        // surface later as an out-of-bounds panic (or silently ragged output).
        assert_eq!(
            cells.len(),
            self.headers.len(),
            "row arity must match headers"
        );
        self.rows.push(cells);
        self
    }

    /// Render the textual formats. `Json` is not representable here — commands go through
    /// [`render_result`], whose signature forces JSON to come from the data model.
    fn render_text(&self, format: Format, style: Style) -> String {
        match format {
            Format::Table | Format::Json => self.render_aligned(style),
            Format::Plain => self.render_plain(),
        }
    }

    /// Test-only alias while the data commands (#9) migrate onto [`render_result`].
    #[cfg(test)]
    pub fn render(&self, format: Format, style: Style) -> String {
        self.render_text(format, style)
    }

    fn render_aligned(&self, style: Style) -> String {
        // Widths and `{:<width$}` padding must count the same unit: CHARS. `String::len`
        // is bytes and misaligns any non-ASCII cell. (Terminal display width — CJK/emoji
        // double-width — is a known further limitation, out of scope for the foundation.)
        let char_len = |s: &str| s.chars().count();
        let mut widths: Vec<usize> = self.headers.iter().map(|h| char_len(h)).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = widths[i].max(char_len(cell));
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

/// Serialize a command's data model as pretty JSON (the `--json` path), newline-terminated
/// — `--json` output is a complete text file (POSIX lines end in `\n`), and `jq`-style
/// consumers and shell prompts expect the terminator.
pub fn to_json<T: serde::Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(value)? + "\n")
}

/// THE single rendering entry point for list-shaped data commands: dispatches `--json` to
/// the command's serde data model and everything else to the curated table — so no command
/// author can ever render JSON from re-encoded table cells, by construction.
/// Single-record commands use [`render_record`].
pub fn render_result<T: serde::Serialize>(
    model: &T,
    table: &Table,
    opts: RenderOptions,
) -> anyhow::Result<String> {
    match opts.format {
        Format::Json => to_json(model),
        other => Ok(table.render_text(other, opts.style)),
    }
}

/// The single-record sibling of [`render_result`]: a vertical key/value layout for
/// commands returning one document (`personal-info`). Same dispatch rule — `--json`
/// serializes the data model; the text formats render sanitized label/value lines
/// (`Plain` = tab-separated for scripts).
pub fn render_record<T: serde::Serialize>(
    model: &T,
    fields: &[(&str, String)],
    opts: RenderOptions,
) -> anyhow::Result<String> {
    if matches!(opts.format, Format::Json) {
        return to_json(model);
    }
    let label_width = fields
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    for (label, value) in fields {
        let value = sanitize(value);
        match opts.format {
            Format::Table => {
                let padded = format!("{label:<label_width$}");
                out.push_str(&opts.style.dim(&padded));
                out.push_str("  ");
                out.push_str(&value);
            }
            Format::Plain => {
                out.push_str(label);
                out.push('\t');
                out.push_str(&value);
            }
            Format::Json => unreachable!("handled above"),
        }
        out.push('\n');
    }
    Ok(out)
}

/// Replace control characters with spaces: rendered output must never carry terminal
/// escape sequences or forge lines/fields in line-oriented formats.
///
/// Scope: Unicode `Cc` (C0 incl. ESC/LF/TAB, DEL, and C1 0x80–0x9F) — everything that can
/// start an escape sequence or break line/field structure. Format characters (`Cf`: bidi
/// overrides, zero-width joiners) are deliberately out of scope: they cannot forge
/// structure or escapes; their only effect is cosmetic glyph reordering of the user's own
/// data on their own terminal.
pub fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
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
    fn non_ascii_cells_align_by_char_count() {
        // The multibyte cell must be the column's WIDTH DRIVER, or byte- and char-counting
        // produce identical output and the test can't catch a regression: "café" is
        // 4 chars / 5 bytes, and every other cell in its column is narrower.
        let mut t = Table::new(["T", "N"]);
        t.row(["café", "1"]).row(["x", "2"]);
        let rendered = t.render(Format::Table, Style::new(false));
        // Char width = 4; a byte-based width (5) would pad one extra column.
        assert_eq!(rendered, "T     N\ncafé  1\nx     2\n");
    }

    #[test]
    fn cells_are_sanitized_against_escape_and_forgery_bytes() {
        let mut t = Table::new(["TAG"]);
        t.row(["evil\x1b[2J\ntag\tx"]);
        let table = t.render(Format::Table, Style::new(true));
        // The ONLY escapes present are our own header bold — none from cell content.
        assert_eq!(
            table.matches('\x1b').count(),
            2,
            "only the header SGR pair: {table:?}"
        );
        let plain = t.render(Format::Plain, Style::new(false));
        assert_eq!(
            plain.lines().count(),
            1,
            "embedded newline must not forge a row"
        );
        assert_eq!(
            plain.matches('\t').count(),
            0,
            "single-column row: embedded tab must not forge a column"
        );
    }

    #[test]
    fn render_result_dispatches_json_to_the_data_model() {
        #[derive(serde::Serialize)]
        struct Day {
            day: &'static str,
        }
        let model = vec![Day { day: "2026-07-01" }];
        let mut t = Table::new(["DAY"]);
        t.row(["2026-07-01"]);

        let json_opts = RenderOptions {
            format: Format::Json,
            style: Style::new(false),
        };
        let json = render_result(&model, &t, json_opts).unwrap();
        assert!(
            json.contains("\"day\": \"2026-07-01\""),
            "JSON comes from the model"
        );

        let plain_opts = RenderOptions {
            format: Format::Plain,
            style: Style::new(false),
        };
        assert_eq!(
            render_result(&model, &t, plain_opts).unwrap(),
            "2026-07-01\n"
        );
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
