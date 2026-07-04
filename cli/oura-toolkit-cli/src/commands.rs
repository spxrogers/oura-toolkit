//! The data commands (#9): each fetches through the generated client (auth + 401-retry +
//! pagination from `api.rs`), then renders through the output layer's single entry point —
//! commands return the rendered string (testable end-to-end against wiremock) and `main`
//! owns the one write to stdout.
//!
//! Fetching and rendering are split: the `fetch_*` functions return the GENERATED models
//! and are the shared data plane for BOTH consumers — the CLI's rendered commands here and
//! the MCP tools (#10), which serialize the same models to JSON. One auth layer, one data
//! plane, two presentations.

use anyhow::{Context, Result};
use oura_toolkit_api::types;
use oura_toolkit_auth::TokenManager;

use crate::api::{paginate, with_auth_retry, DateRange};
use crate::output::{render_record, render_result, RenderOptions, Table};

/// Everything a data command needs; `base_url` is injected so tests point at wiremock.
pub struct Ctx {
    pub manager: TokenManager,
    pub base_url: String,
    pub render: RenderOptions,
}

/// Render an optional field for a table cell: its `Display` form, or `-` when absent.
fn opt<T: std::fmt::Display>(v: &Option<T>) -> String {
    v.as_ref().map(T::to_string).unwrap_or_else(|| "-".into())
}

pub async fn fetch_sleep(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicDailySleep>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_daily_sleep_documents_v2_usercollection_daily_sleep_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching daily sleep")
}

pub async fn sleep(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_sleep(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "SCORE", "DEEP", "REM", "EFFICIENCY"]);
    for d in &docs {
        let c = &d.contributors;
        table.row([
            d.day.to_string(),
            opt(&d.score),
            opt(&c.deep_sleep),
            opt(&c.rem_sleep),
            opt(&c.efficiency),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_readiness(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicDailyReadiness>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_daily_readiness_documents_v2_usercollection_daily_readiness_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching daily readiness")
}

pub async fn readiness(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_readiness(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "SCORE", "TEMP DEV"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt(&d.score),
            opt(&d.temperature_deviation),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_activity(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicDailyActivity>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_daily_activity_documents_v2_usercollection_daily_activity_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching daily activity")
}

pub async fn activity(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_activity(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "SCORE", "STEPS", "ACTIVE CAL"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt(&d.score),
            d.steps.to_string(),
            d.active_calories.to_string(),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_stress(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicDailyStress>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_daily_stress_documents_v2_usercollection_daily_stress_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching daily stress")
}

pub async fn stress(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_stress(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "SUMMARY", "STRESS HIGH", "RECOVERY HIGH"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt(&d.day_summary),
            opt(&d.stress_high),
            opt(&d.recovery_high),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_heartrate(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicHeartRateRow>> {
    let (start, end) = range.as_utc_bounds();
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| {
        async move {
            let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
                let token = token.clone();
                async move {
                    client
                        .multiple_heartrate_documents_v2_usercollection_heartrate_get(
                            Some(&end),
                            None,
                            None,
                            token.as_deref(),
                            Some(&start),
                        )
                        .await
                }
            })
            .await?;
            let inner = resp.into_inner();
            // The heartrate envelope is an anyOf wrapper; the time-series arm carries
            // the rows. An absent arm is an empty page.
            match inner.subtype_0 {
                Some(series) => Ok((series.data, series.next_token)),
                None => Ok((Vec::new(), None)),
            }
        }
    })
    .await
    .context("fetching heart rate")
}

pub async fn heartrate(ctx: &Ctx, range: DateRange) -> Result<String> {
    let rows = fetch_heartrate(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["TIMESTAMP", "BPM", "SOURCE"]);
    for r in &rows {
        table.row([
            r.timestamp.to_string(),
            r.bpm.to_string(),
            r.source.to_string(),
        ]);
    }
    render_result(&rows, &table, ctx.render)
}

pub async fn fetch_sessions(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicSession>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_session_documents_v2_usercollection_session_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching sessions")
}

pub async fn sessions(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_sessions(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "TYPE", "START", "END"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            d.type_.to_string(),
            d.start_datetime.to_string(),
            d.end_datetime.to_string(),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_workouts(
    manager: &TokenManager,
    base_url: &str,
    range: DateRange,
) -> Result<Vec<types::PublicWorkout>> {
    let (start, end) = (range.start, range.end);
    let rate_budget = crate::api::RateLimitBudget::new();
    let rate_budget = &rate_budget;
    paginate(|token| async move {
        let resp = with_auth_retry(manager, base_url, rate_budget, |client| {
            let token = token.clone();
            async move {
                client
                    .multiple_workout_documents_v2_usercollection_workout_get(
                        Some(&end),
                        None,
                        token.as_deref(),
                        Some(&start),
                    )
                    .await
            }
        })
        .await?;
        let inner = resp.into_inner();
        Ok((inner.data, inner.next_token))
    })
    .await
    .context("fetching workouts")
}

pub async fn workouts(ctx: &Ctx, range: DateRange) -> Result<String> {
    let docs = fetch_workouts(&ctx.manager, &ctx.base_url, range).await?;

    let mut table = Table::new(["DAY", "ACTIVITY", "INTENSITY", "CALORIES"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            d.activity.clone(),
            d.intensity.to_string(),
            opt(&d.calories),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn fetch_personal_info(
    manager: &TokenManager,
    base_url: &str,
) -> Result<types::PersonalInfoResponse> {
    let rate_budget = crate::api::RateLimitBudget::new();
    Ok(
        with_auth_retry(manager, base_url, &rate_budget, |client| async move {
            client
                .single_personal_info_document_v2_usercollection_personal_info_get()
                .await
        })
        .await
        .context("fetching personal info")?
        .into_inner(),
    )
}

pub async fn personal_info(ctx: &Ctx) -> Result<String> {
    let info = fetch_personal_info(&ctx.manager, &ctx.base_url).await?;

    let fields = [
        ("Age", opt(&info.age)),
        ("Biological sex", opt(&info.biological_sex)),
        ("Height (m)", opt(&info.height)),
        ("Weight (kg)", opt(&info.weight)),
        ("Email", opt(&info.email)),
    ];
    render_record(&info, &fields, ctx.render)
}
