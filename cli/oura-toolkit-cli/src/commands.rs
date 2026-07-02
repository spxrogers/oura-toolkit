//! The data commands (#9): each fetches through the generated client (auth + 401-retry +
//! pagination from `api.rs`), then renders through the output layer's single entry point —
//! commands return the rendered string (testable end-to-end against wiremock) and `main`
//! owns the one `println!`.

use anyhow::{Context, Result};
use oura_toolkit_auth::TokenManager;

use crate::api::{paginate, with_auth_retry, DateRange};
use crate::output::{render_record, render_result, RenderOptions, Table};

/// Everything a data command needs; `base_url` is injected so tests point at wiremock.
pub struct Ctx {
    pub manager: TokenManager,
    pub base_url: String,
    pub render: RenderOptions,
}

fn opt_num<T: std::fmt::Display>(v: &Option<T>) -> String {
    v.as_ref().map(T::to_string).unwrap_or_else(|| "-".into())
}

pub async fn sleep(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching daily sleep")?;

    let mut table = Table::new(["DAY", "SCORE", "DEEP", "REM", "EFFICIENCY"]);
    for d in &docs {
        let c = &d.contributors;
        table.row([
            d.day.to_string(),
            opt_num(&d.score),
            opt_num(&c.deep_sleep),
            opt_num(&c.rem_sleep),
            opt_num(&c.efficiency),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn readiness(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching daily readiness")?;

    let mut table = Table::new(["DAY", "SCORE", "TEMP DEV"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt_num(&d.score),
            opt_num(&d.temperature_deviation),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn activity(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching daily activity")?;

    let mut table = Table::new(["DAY", "SCORE", "STEPS", "ACTIVE CAL"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt_num(&d.score),
            d.steps.to_string(),
            d.active_calories.to_string(),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn stress(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching daily stress")?;

    let mut table = Table::new(["DAY", "SUMMARY", "STRESS HIGH", "RECOVERY HIGH"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            opt_num(&d.day_summary),
            opt_num(&d.stress_high),
            opt_num(&d.recovery_high),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn heartrate(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = range.as_utc_bounds();
    let rows = paginate(|token| {
        async move {
            let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching heart rate")?;

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

pub async fn sessions(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching sessions")?;

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

pub async fn workouts(ctx: &Ctx, range: DateRange) -> Result<String> {
    let (start, end) = (range.start, range.end);
    let docs = paginate(|token| async move {
        let resp = with_auth_retry(&ctx.manager, &ctx.base_url, |client| {
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
    .context("fetching workouts")?;

    let mut table = Table::new(["DAY", "ACTIVITY", "INTENSITY", "CALORIES"]);
    for d in &docs {
        table.row([
            d.day.to_string(),
            d.activity.clone(),
            d.intensity.to_string(),
            opt_num(&d.calories),
        ]);
    }
    render_result(&docs, &table, ctx.render)
}

pub async fn personal_info(ctx: &Ctx) -> Result<String> {
    let info = with_auth_retry(&ctx.manager, &ctx.base_url, |client| async move {
        client
            .single_personal_info_document_v2_usercollection_personal_info_get()
            .await
    })
    .await
    .context("fetching personal info")?
    .into_inner();

    let fields = [
        ("Age", opt_num(&info.age)),
        ("Biological sex", opt_num(&info.biological_sex)),
        ("Height (m)", opt_num(&info.height)),
        ("Weight (kg)", opt_num(&info.weight)),
        ("Email", opt_num(&info.email)),
    ];
    render_record(&info, &fields, ctx.render)
}
