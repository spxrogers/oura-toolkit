//! `oura dashboard` — the local desktop GUI: one SELF-CONTAINED HTML file rendered
//! from the day-grain store and opened in the default browser.
//!
//! Design constraints (same weight as the MCP stdout rule):
//! - **Zero egress**: no external scripts, styles, fonts, or images — everything is
//!   inline, so opening the file makes no network request of any kind (test-enforced:
//!   the output contains no `http://`/`https://` substring). The dashboard inherits
//!   the store's privacy properties because it never leaves the machine.
//! - **No server**: a static file, not a localhost daemon — nothing to keep running,
//!   no port, no new attack surface.
//! - **Rust computes, the page displays**: every number, path point, and tooltip
//!   label is precomputed here; the inline JS only moves a crosshair and shows
//!   prebuilt labels.
//!
//! Chart method (dataviz skill): small multiples with ONE axis each (never dual-axis);
//! raw daily values recede (muted, thin) while the 7-day moving average carries the
//! hue; habit charts plot the trailing-28-day rate — the long-grain consistency line
//! the habit feature exists for. Colors are the validated reference palette with
//! selected dark-mode steps behind `prefers-color-scheme`; every chart ships a
//! crosshair tooltip and a weekly-means table view.

use chrono::{Datelike as _, NaiveDate};

use oura_toolkit_health::engine::{self, CapacityBand};
use oura_toolkit_health::habits;
use oura_toolkit_health::{DayMap, DayRecord};

/// Trend charts cover this trailing window (26 weeks — matches the calendar
/// importer's recurrence horizon).
pub const TREND_WINDOW_DAYS: u32 = 182;

/// Habit rate lines use the 28-day trailing window (the headline consistency number).
pub const HABIT_RATE_WINDOW: u32 = 28;

// ---- geometry (viewBox units) ----
const W: f64 = 560.0;
const H: f64 = 220.0;
const ML: f64 = 46.0; // left margin (y tick labels)
const MR: f64 = 10.0;
const MT: f64 = 10.0;
const MB: f64 = 26.0; // bottom margin (month labels)

/// Escape text for HTML text/attribute positions. Everything user- or data-derived
/// goes through here (habit names are already normalized, but escaping stays
/// unconditional — defense does not depend on upstream sanitization holding).
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c if c.is_control() => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

/// One metric extracted from the store for charting.
struct Metric {
    title: &'static str,
    unit: &'static str,
    /// CSS var of the accent hue (fixed categorical slot — color follows the metric).
    accent: &'static str,
    daily: Vec<(NaiveDate, f64)>,
}

fn collect<F: Fn(&DayRecord) -> Option<f64>>(
    days: &DayMap,
    from: NaiveDate,
    f: F,
) -> Vec<(NaiveDate, f64)> {
    days.range(from..)
        .filter_map(|(d, rec)| f(rec).map(|v| (*d, v)))
        .collect()
}

/// Trailing 7-day moving average over the AVAILABLE values in each window (gaps are
/// common in real day-grain data; an empty window yields no point).
fn moving_average(daily: &[(NaiveDate, f64)], window_days: u32) -> Vec<(NaiveDate, f64)> {
    let mut out = Vec::with_capacity(daily.len());
    for (i, (day, _)) in daily.iter().enumerate() {
        let start = *day - chrono::Days::new(u64::from(window_days) - 1);
        let mut sum = 0.0;
        let mut n = 0u32;
        for (d, v) in daily[..=i].iter().rev() {
            if *d < start {
                break;
            }
            sum += v;
            n += 1;
        }
        if n > 0 {
            out.push((*day, sum / f64::from(n)));
        }
    }
    out
}

/// Nice tick step: 1/2/5 × 10^k covering the span with ~4 ticks.
fn nice_step(span: f64) -> f64 {
    let raw = span / 4.0;
    let mag = 10f64.powf(raw.log10().floor());
    for m in [1.0, 2.0, 5.0, 10.0] {
        if raw <= m * mag {
            return m * mag;
        }
    }
    10.0 * mag
}

struct Scale {
    x0: NaiveDate,
    x_span_days: f64,
    y_min: f64,
    y_max: f64,
}

impl Scale {
    fn x(&self, d: NaiveDate) -> f64 {
        let t = (d - self.x0).num_days() as f64 / self.x_span_days;
        ML + t * (W - ML - MR)
    }
    fn y(&self, v: f64) -> f64 {
        let t = (v - self.y_min) / (self.y_max - self.y_min);
        H - MB - t * (H - MB - MT)
    }
}

/// Polyline path split at gaps of more than 3 days — drawing a line across a
/// two-week hole would invent data.
fn path(points: &[(NaiveDate, f64)], s: &Scale) -> String {
    let mut d = String::new();
    let mut prev: Option<NaiveDate> = None;
    for (day, v) in points {
        let cmd = match prev {
            Some(p) if (*day - p).num_days() <= 3 => 'L',
            _ => 'M',
        };
        d.push_str(&format!("{cmd}{:.1} {:.1} ", s.x(*day), s.y(*v)));
        prev = Some(*day);
    }
    d.trim_end().to_string()
}

/// Everything one small-multiple chart needs.
struct ChartSpec<'a> {
    dom_id: String,
    title: &'a str,
    unit: &'a str,
    /// CSS var of the accent hue (fixed slot — color follows the metric).
    accent: &'a str,
    daily: &'a [(NaiveDate, f64)],
    ma: &'a [(NaiveDate, f64)],
    ma_label: &'a str,
    decimals: usize,
}

/// Render one small-multiple chart: muted daily line + accent moving-average line,
/// month gridlines, ~4 y ticks, a crosshair layer, and a weekly-means table view.
fn chart(spec: &ChartSpec<'_>) -> String {
    let ChartSpec {
        dom_id,
        title,
        unit,
        accent,
        daily,
        ma,
        ma_label,
        decimals,
    } = spec;
    let (daily, ma, decimals) = (*daily, *ma, *decimals);
    let (min_d, max_d) = (daily[0].0, daily[daily.len() - 1].0);
    let values = daily
        .iter()
        .map(|(_, v)| *v)
        .chain(ma.iter().map(|(_, v)| *v));
    let (mut y_min, mut y_max) =
        values.fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));
    if (y_max - y_min).abs() < 1e-9 {
        y_min -= 1.0;
        y_max += 1.0;
    }
    let pad = (y_max - y_min) * 0.08;
    let step = nice_step(y_max - y_min);
    let y_lo = ((y_min - pad) / step).floor() * step;
    let y_hi = ((y_max + pad) / step).ceil() * step;
    let scale = Scale {
        x0: min_d,
        x_span_days: ((max_d - min_d).num_days() as f64).max(1.0),
        y_min: y_lo,
        y_max: y_hi,
    };

    // Y gridlines + labels.
    let mut grid = String::new();
    let mut tick = y_lo;
    while tick <= y_hi + 1e-9 {
        let y = scale.y(tick);
        grid.push_str(&format!(
            r##"<line class="grid" x1="{ML}" x2="{x2}" y1="{y:.1}" y2="{y:.1}"/><text class="tick" x="{tx}" y="{ty:.1}" text-anchor="end">{label}</text>"##,
            x2 = W - MR,
            tx = ML - 6.0,
            ty = y + 3.5,
            label = format_num(tick, decimals),
        ));
        tick += step;
    }
    // X ticks at month starts; the range's first day is labeled too, unless a month
    // start lands within 2 weeks (the labels would collide).
    let mut month_starts: Vec<NaiveDate> = Vec::new();
    let mut d = min_d;
    while d <= max_d {
        if d.day() == 1 {
            month_starts.push(d);
        }
        d = d.succ_opt().expect("date in range");
    }
    let label_min = month_starts
        .first()
        .is_none_or(|first| (*first - min_d).num_days() >= 14);
    let ticks = month_starts
        .iter()
        .copied()
        .chain(label_min.then_some(min_d));
    for tick_day in ticks {
        grid.push_str(&format!(
            r##"<text class="tick" x="{x:.1}" y="{y}" text-anchor="middle">{m}</text>"##,
            x = scale.x(tick_day),
            y = H - 8.0,
            m = month_abbr(tick_day.month()),
        ));
    }

    // Tooltip points: precomputed labels (Rust computes; JS displays).
    let ma_lookup: std::collections::BTreeMap<NaiveDate, f64> = ma.iter().copied().collect();
    let points_json: Vec<String> = daily
        .iter()
        .map(|(day, v)| {
            let ma_part = ma_lookup
                .get(day)
                .map(|m| format!(" · {ma_label} {}", format_num(*m, decimals)))
                .unwrap_or_default();
            format!(
                r#"{{"x":{:.1},"label":"{day} · {}{}"}}"#,
                scale.x(*day),
                format_num(*v, decimals),
                ma_part
            )
        })
        .collect();

    // Weekly-means table view (the accessibility channel; 26 rows, not 182).
    let mut weeks: std::collections::BTreeMap<NaiveDate, (f64, u32)> =
        std::collections::BTreeMap::new();
    for (day, v) in daily {
        let ws = engine::week_start(*day);
        let e = weeks.entry(ws).or_insert((0.0, 0));
        e.0 += v;
        e.1 += 1;
    }
    let rows: String = weeks
        .iter()
        .map(|(ws, (sum, n))| {
            format!(
                "<tr><td>{ws}</td><td>{}</td></tr>",
                format_num(sum / f64::from(*n), decimals)
            )
        })
        .collect();

    format!(
        r##"<figure class="chart" style="--accent:{accent}">
<figcaption>{title} <span class="unit">{unit}</span><span class="legend"><i class="sw raw"></i> daily <i class="sw ma"></i> {ma_label}</span></figcaption>
<div class="plot">
<svg viewBox="0 0 {W} {H}" role="img" aria-label="{title}">
{grid}
<path class="raw" d="{raw}"/>
<path class="ma" d="{map}"/>
<line class="cross" x1="0" x2="0" y1="{MT}" y2="{yb}" style="display:none"/>
</svg>
<div class="tip" style="display:none"></div>
</div>
<script type="application/json" class="chart-data" id="{dom_id}">{{"points":[{pts}]}}</script>
<details><summary>data table (weekly means)</summary><table><thead><tr><th>week of</th><th>{title}</th></tr></thead><tbody>{rows}</tbody></table></details>
</figure>"##,
        title = esc(title),
        unit = esc(unit),
        raw = path(daily, &scale),
        map = path(ma, &scale),
        yb = H - MB,
        pts = points_json.join(","),
    )
}

fn format_num(v: f64, decimals: usize) -> String {
    format!("{v:.decimals$}")
}

fn month_abbr(m: u32) -> &'static str {
    [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ][(m as usize) - 1]
}

/// One source's coverage: (name, day count, first/last dates).
type SourceCoverage = (&'static str, u32, Option<(NaiveDate, NaiveDate)>);
/// Probe for whether a record carries a given source's slot.
type SlotProbe = fn(&DayRecord) -> bool;

/// Per-source coverage: (name, day count, first, last).
fn coverage(days: &DayMap) -> Vec<SourceCoverage> {
    let slots: [(&'static str, SlotProbe); 5] = [
        ("Oura", |r| r.oura.is_some()),
        ("Apple Health", |r| r.apple.is_some()),
        ("Calendar", |r| r.calendar.is_some()),
        ("Toggl", |r| r.toggl.is_some()),
        ("Habits", |r| r.habits.is_some()),
    ];
    slots
        .into_iter()
        .map(|(name, has)| {
            let mut count = 0u32;
            let mut first_last: Option<(NaiveDate, NaiveDate)> = None;
            for (d, rec) in days {
                if has(rec) {
                    count += 1;
                    first_last = Some(match first_last {
                        None => (*d, *d),
                        Some((f, _)) => (f, *d),
                    });
                }
            }
            (name, count, first_last)
        })
        .collect()
}

/// Render the full dashboard HTML from the store contents. Pure and deterministic
/// (`today` injected) — the whole page is testable as a string.
pub fn render(days: &DayMap, today: NaiveDate, store_path: &str) -> String {
    let from = today - chrono::Days::new(u64::from(TREND_WINDOW_DAYS) - 1);

    // Fixed categorical slots per metric (color follows the metric, never its rank).
    let metrics = [
        Metric {
            title: "Readiness",
            unit: "score",
            accent: "var(--s1)",
            daily: collect(days, from, |r| {
                r.oura.as_ref().and_then(|o| o.readiness_score)
            }),
        },
        Metric {
            title: "Sleep score",
            unit: "score",
            accent: "var(--s2)",
            daily: collect(days, from, |r| r.oura.as_ref().and_then(|o| o.sleep_score)),
        },
        Metric {
            title: "Sleep (Apple Watch)",
            unit: "hours/night",
            accent: "var(--s5)",
            daily: collect(days, from, |r| {
                r.apple
                    .as_ref()
                    .and_then(|a| a.sleep_minutes)
                    .map(|m| m / 60.0)
            }),
        },
        Metric {
            title: "HRV SDNN (Apple Watch)",
            unit: "ms",
            accent: "var(--s4)",
            daily: collect(days, from, |r| r.apple.as_ref().and_then(|a| a.hrv_sdnn_ms)),
        },
        Metric {
            title: "Meeting load",
            unit: "hours/day",
            accent: "var(--s6)",
            daily: collect(days, from, |r| r.calendar.as_ref().map(|c| c.meeting_hours)),
        },
        Metric {
            title: "Tracked time (Toggl)",
            unit: "hours/day",
            accent: "var(--s8)",
            daily: collect(days, from, |r| r.toggl.as_ref().map(|t| t.tracked_hours)),
        },
    ];

    let mut trend_charts = String::new();
    for (i, m) in metrics.iter().enumerate() {
        if m.daily.len() < 2 {
            continue; // nothing to draw; the coverage cards make the gap visible
        }
        let ma = moving_average(&m.daily, 7);
        trend_charts.push_str(&chart(&ChartSpec {
            dom_id: format!("trend-{i}"),
            title: m.title,
            unit: m.unit,
            accent: m.accent,
            daily: &m.daily,
            ma: &ma,
            ma_label: "7-day avg",
            decimals: 1,
        }));
    }

    // Capacity tile (status colors: band → good/warning/critical).
    let capacity_html = match engine::capacity(days, today) {
        Ok(report) => {
            let class = match report.band {
                CapacityBand::Comfortable => "good",
                CapacityBand::Stretched => "warning",
                CapacityBand::Overloaded => "critical",
            };
            let components: String = report
                .components
                .iter()
                .map(|c| {
                    format!(
                        "<li><b>{}</b> −{} pts — {}</li>",
                        esc(c.name),
                        format_num(c.points, 1),
                        esc(&c.detail)
                    )
                })
                .collect();
            format!(
                r##"<div class="tile {class}"><div class="hero">{}%</div><div class="band">{} · week of {}</div><ul class="components">{components}</ul></div>"##,
                report.capacity_pct, report.band, report.week.week_start
            )
        }
        Err(e) => format!(
            r##"<div class="tile muted"><div class="band">capacity unavailable</div><p>{}</p></div>"##,
            esc(&e.to_string())
        ),
    };

    // Coverage cards.
    let coverage_html: String = coverage(days)
        .into_iter()
        .map(|(name, count, span)| {
            let detail = match span {
                Some((first, last)) => format!("{count} days · {first} → {last}"),
                None => "no data imported".to_string(),
            };
            format!(r##"<div class="card"><b>{name}</b><span>{detail}</span></div>"##)
        })
        .collect();

    // Habits: stats table (its own block, NOT a grid cell — a table stretched to a
    // chart cell's height reads terribly) + one rate chart per habit in the grid
    // (single series each — the title carries identity, so no per-habit hue juggling
    // and never a cycled palette).
    let stats = habits::habit_stats(days, today);
    let (habit_table, habit_charts) = if stats.is_empty() {
        (
            r#"<p class="muted-text">No habits logged yet — <code>oura habit log &lt;name&gt;</code> starts one; consistency shows up here as a moving-average rate, not a streak.</p>"#.to_string(),
            String::new(),
        )
    } else {
        let rows: String = stats
            .iter()
            .map(|s| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td><b>{}</b></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    esc(&s.name),
                    format_num(s.rate_7d, 1),
                    format_num(s.rate_28d, 1),
                    format_num(s.rate_91d, 1),
                    s.total_days,
                    s.last_logged,
                )
            })
            .collect();
        let table = format!(
            r##"<table class="habit-table"><thead><tr><th>habit</th><th>7d ×/wk</th><th>28d ×/wk</th><th>91d ×/wk</th><th>days</th><th>last</th></tr></thead><tbody>{rows}</tbody></table>"##
        );
        let mut charts_html = String::new();
        for (i, s) in stats.iter().enumerate() {
            let series = habits::habit_rate_series(days, &s.name, today, HABIT_RATE_WINDOW);
            if series.len() < 2 {
                continue;
            }
            // The rate series IS already a moving average; plot it as the accent line
            // with the 7d-window rate as the receding raw layer.
            let raw = habits::habit_rate_series(days, &s.name, today, 7);
            charts_html.push_str(&chart(&ChartSpec {
                dom_id: format!("habit-{i}"),
                title: &s.name,
                unit: "days/week",
                accent: "var(--s1)",
                daily: &raw,
                ma: &series,
                ma_label: "28-day rate",
                decimals: 1,
            }));
        }
        (table, charts_html)
    };

    let tooltip_js = r##"document.querySelectorAll('.chart').forEach(function(chart){
  var dataEl = chart.querySelector('script.chart-data');
  var svg = chart.querySelector('svg');
  var tip = chart.querySelector('.tip');
  var cross = chart.querySelector('.cross');
  if (!dataEl || !svg) return;
  var points = JSON.parse(dataEl.textContent).points;
  svg.addEventListener('pointermove', function(e){
    var r = svg.getBoundingClientRect();
    var fx = (e.clientX - r.left) / r.width * 560;
    var best = null;
    for (var i = 0; i < points.length; i++) {
      var d = Math.abs(points[i].x - fx);
      if (!best || d < best.d) best = {d: d, p: points[i]};
    }
    if (!best) return;
    cross.setAttribute('x1', best.p.x); cross.setAttribute('x2', best.p.x);
    cross.style.display = 'block';
    tip.textContent = best.p.label;
    tip.style.display = 'block';
    var left = best.p.x / 560 * r.width;
    tip.style.left = Math.max(4, Math.min(left + 10, r.width - 170)) + 'px';
  });
  svg.addEventListener('pointerleave', function(){
    cross.style.display = 'none'; tip.style.display = 'none';
  });
});"##;

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>oura-toolkit dashboard</title>
<style>
:root {{
  --surface: #fcfcfb; --page: #f9f9f7;
  --ink: #0b0b0b; --ink-2: #52514e; --muted: #898781;
  --grid: #e1e0d9; --axis: #c3c2b7; --border: rgba(11,11,11,0.10);
  --s1: #2a78d6; --s2: #1baf7a; --s4: #008300; --s5: #4a3aa7; --s6: #e34948; --s8: #eb6834;
  --good: #0ca30c; --warning: #fab219; --critical: #d03b3b;
}}
@media (prefers-color-scheme: dark) {{
  :root {{
    --surface: #1a1a19; --page: #0d0d0d;
    --ink: #ffffff; --ink-2: #c3c2b7; --muted: #898781;
    --grid: #2c2c2a; --axis: #383835; --border: rgba(255,255,255,0.10);
    --s1: #3987e5; --s2: #199e70; --s4: #008300; --s5: #9085e9; --s6: #e66767; --s8: #d95926;
  }}
}}
* {{ box-sizing: border-box; margin: 0; }}
body {{ background: var(--page); color: var(--ink); font: 15px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; padding: 24px; }}
main {{ max-width: 1180px; margin: 0 auto; }}
h1 {{ font-size: 20px; }} h2 {{ font-size: 16px; margin: 28px 0 10px; }}
.meta {{ color: var(--muted); font-size: 12px; margin-bottom: 16px; }}
.cards {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 10px; }}
.card {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 10px 12px; display: flex; flex-direction: column; }}
.card span {{ color: var(--ink-2); font-size: 12px; }}
.tile {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 14px 16px; }}
.tile .hero {{ font-size: 40px; font-weight: 700; }}
.tile.good .hero {{ color: var(--good); }} .tile.warning .hero {{ color: var(--warning); }} .tile.critical .hero {{ color: var(--critical); }}
.tile .band {{ color: var(--ink-2); }}
.tile ul {{ margin: 8px 0 0 18px; color: var(--ink-2); font-size: 13px; }}
.charts {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(430px, 1fr)); gap: 14px; }}
figure.chart {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 10px 12px; }}
figcaption {{ font-weight: 600; font-size: 13px; }}
figcaption .unit {{ color: var(--muted); font-weight: 400; }}
figcaption .legend {{ float: right; color: var(--ink-2); font-weight: 400; font-size: 12px; }}
.sw {{ display: inline-block; width: 10px; height: 3px; vertical-align: middle; border-radius: 2px; }}
.sw.raw {{ background: var(--muted); opacity: .6; }} .sw.ma {{ background: var(--accent); }}
.plot {{ position: relative; }}
svg {{ width: 100%; height: auto; display: block; }}
svg .grid {{ stroke: var(--grid); stroke-width: 1; }}
svg .tick {{ fill: var(--muted); font-size: 10px; font-variant-numeric: tabular-nums; }}
svg path {{ fill: none; }}
svg path.raw {{ stroke: var(--muted); stroke-width: 1; opacity: .55; }}
svg path.ma {{ stroke: var(--accent); stroke-width: 2; }}
svg .cross {{ stroke: var(--axis); stroke-width: 1; }}
.tip {{ position: absolute; top: 6px; background: var(--surface); border: 1px solid var(--border); border-radius: 6px; padding: 3px 8px; font-size: 12px; color: var(--ink-2); pointer-events: none; white-space: nowrap; }}
details {{ margin-top: 6px; font-size: 12px; color: var(--ink-2); }}
table {{ border-collapse: collapse; margin-top: 6px; font-variant-numeric: tabular-nums; }}
td, th {{ border-bottom: 1px solid var(--grid); padding: 3px 10px 3px 0; text-align: left; font-size: 12px; }}
.habit-table {{ margin-bottom: 14px; }}
.muted-text {{ color: var(--ink-2); }}
code {{ background: var(--surface); border: 1px solid var(--border); border-radius: 4px; padding: 0 4px; }}
</style>
</head>
<body>
<main>
<h1>oura-toolkit dashboard</h1>
<p class="meta">generated {today} · store: {store} · this file is self-contained and loads nothing from the network</p>
<h2>Capacity</h2>
{capacity}
<h2>Sources</h2>
<div class="cards">{coverage}</div>
<h2>Trends <span class="meta">last {window} days · line = 7-day average, faint = daily</span></h2>
<div class="charts">{trends}</div>
<h2>Habits <span class="meta">rates are days/week over trailing windows — consistency, not streaks</span></h2>
{habit_table}
<div class="charts habits">{habit_charts}</div>
</main>
<script>{js}</script>
</body>
</html>
"##,
        today = today,
        store = esc(store_path),
        capacity = capacity_html,
        coverage = coverage_html,
        window = TREND_WINDOW_DAYS,
        trends = trend_charts,
        habit_table = habit_table,
        habit_charts = habit_charts,
        js = tooltip_js,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oura_toolkit_health::{CalendarDay, HabitsDay, OuraDay};

    fn d(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    fn sample_days() -> DayMap {
        let mut days = DayMap::new();
        for i in 0..30 {
            let date = d("2026-06-01") + chrono::Days::new(i);
            let mut rec = DayRecord {
                oura: Some(OuraDay {
                    readiness_score: Some(70.0 + (i % 10) as f64),
                    sleep_score: Some(80.0),
                    ..Default::default()
                }),
                calendar: Some(CalendarDay {
                    meeting_hours: 4.0,
                    event_count: 5,
                    ..Default::default()
                }),
                ..Default::default()
            };
            if i % 2 == 0 {
                rec.habits = Some(HabitsDay {
                    done: std::collections::BTreeSet::from(["exercise".to_string()]),
                });
            }
            days.insert(date, rec);
        }
        days
    }

    #[test]
    fn dashboard_is_fully_self_contained() {
        let html = render(&sample_days(), d("2026-06-30"), "/tmp/store");
        // The zero-egress guarantee: NOTHING in the page references the network.
        // (This is what makes "the dashboard inherits the store's privacy" true.)
        for needle in ["http://", "https://", "//cdn", "@import", "url("] {
            assert!(
                !html.contains(needle),
                "dashboard must not reference the network: found {needle:?}"
            );
        }
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn dashboard_renders_sections_from_the_store() {
        let html = render(&sample_days(), d("2026-06-30"), "/tmp/store");
        assert!(html.contains("Readiness"), "trend chart for readiness");
        assert!(html.contains("Meeting load"), "calendar trend");
        assert!(
            !html.contains("Tracked time"),
            "no toggl data → no toggl chart"
        );
        assert!(html.contains("30 days"), "coverage counts oura days");
        assert!(
            html.contains("no data imported"),
            "empty sources are visible"
        );
        assert!(html.contains("exercise"), "habit table row");
        assert!(html.contains("days/week"), "habit rate unit");
        assert!(
            html.contains("capacity unavailable") && html.contains("not enough history"),
            "thin history renders the refusal, not a fabricated number"
        );
    }

    #[test]
    fn hostile_store_content_is_escaped() {
        // Habit names are normalized upstream, but the renderer must not DEPEND on
        // that: escape unconditionally (defense in depth, e.g. a hand-edited store).
        let mut days = sample_days();
        days.entry(d("2026-06-15")).or_default().habits = Some(HabitsDay {
            done: std::collections::BTreeSet::from(["<script>alert(1)</script>".to_string()]),
        });
        let html = render(&days, d("2026-06-30"), "/tmp/<script>");
        assert!(
            !html.contains("<script>alert"),
            "store content must be HTML-escaped"
        );
        assert!(html.contains("&lt;script&gt;alert"), "escaped form present");
    }

    #[test]
    fn moving_average_windows_and_gaps() {
        let daily = vec![
            (d("2026-06-01"), 10.0),
            (d("2026-06-02"), 20.0),
            (d("2026-06-10"), 40.0),
        ];
        let ma = moving_average(&daily, 7);
        assert_eq!(ma[0], (d("2026-06-01"), 10.0));
        assert_eq!(ma[1], (d("2026-06-02"), 15.0));
        // 2026-06-10's window (06-04..06-10) excludes the first two points.
        assert_eq!(ma[2], (d("2026-06-10"), 40.0));

        // And the path splits at the 8-day gap instead of drawing across it.
        let scale = Scale {
            x0: d("2026-06-01"),
            x_span_days: 9.0,
            y_min: 0.0,
            y_max: 50.0,
        };
        let p = path(&daily, &scale);
        assert_eq!(
            p.matches('M').count(),
            2,
            "gap > 3 days starts a new segment: {p}"
        );
    }
}
