use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{adapters::ActivitiesListRepository, use_cases};

use crate::prelude::WEEKLY_REPORT_SERVICE_TAG;

/// A report summarizing activities for a specific week.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct WeeklyReport {
    /// The date (a sunday) when the week started in ISO 8601 format (YYYY-MM-DD).
    #[schema(example = "2025-10-12")]
    pub week_start: String,

    /// The date (a saturday) when the week ended in ISO 8601 format (YYYY-MM-DD).
    #[schema(example = "2025-10-18")]
    pub week_end: String,

    /// The total duration of all activities in the week in ISO 8601 format (PT1H).
    #[schema(example = "PT1800S")]
    pub total_duration: String,

    /// A map of accounting category IDs to total duration spent in that category in ISO 8601 format (PT1H).
    #[schema(example = r#"{"category-1": "PT3600S", "category-2": "PT7200S"}"#)]
    pub duration_per_category: HashMap<String, String>,

    /// A nested map where the outer key is the date (YYYY-MM-DD) and the inner map contains
    /// accounting category IDs to total duration spent in that category on that day in ISO 8601 format (PT1H).
    #[schema(
        example = r#"{"2025-10-12": {"category-1": "PT3600S"}, "2025-10-13": {"category-2": "PT7200S"}}"#
    )]
    pub daily_durations_per_category: HashMap<String, HashMap<String, String>>,
}

/// Creates an OpenAPI router for the weekly report service.
///
/// # Arguments
///
/// - `repository`: An `Arc<Mutex<PsqlActivitiesListRepository>>` instance for accessing the activities repository.
///
/// # Returns
///
/// - An `OpenApiRouter` configured with routes and state for the weekly report service.
pub fn router<R>(repository: Arc<Mutex<R>>) -> OpenApiRouter
where
    R: 'static + Send + Sync + ActivitiesListRepository,
{
    OpenApiRouter::new()
        .routes(routes!(generate_weekly_report))
        .with_state(repository)
}

// Query parameters for generating weekly reports.
#[derive(Deserialize, IntoParams)]
struct GenerateWeeklyReportQuery {
    /// The date (a sunday) when the week started in ISO 8601 format (YYYY-MM-DD).
    #[param(example = "2025-10-12")]
    week_start_date: String,
}

/// Generates a weekly report for the specified week starting date.
#[utoipa::path(
    get,
    path = "",
    tag = WEEKLY_REPORT_SERVICE_TAG,
    params(
        GenerateWeeklyReportQuery,
    ),
    responses(
        (status = 201, description = "Weekly report created successfully", body = WeeklyReport)
    )
)]
async fn generate_weekly_report<R>(
    State(store): State<Arc<Mutex<R>>>,
    query: Query<GenerateWeeklyReportQuery>,
) -> impl IntoResponse
where
    R: 'static + Send + Sync + ActivitiesListRepository,
{
    let week_start_date = query.week_start_date.parse().unwrap();
    let store = store.lock().await;
    let weekly_report = use_cases::weekly_report::WeeklyReport::new(week_start_date, &*store).await;

    let daily_durations_per_category = weekly_report
        .daily_durations_per_category()
        .iter()
        .map(|(date, category_map)| {
            (
                date.to_string(),
                category_map
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            )
        })
        .collect();

    let response = WeeklyReport {
        week_start: weekly_report.week_start().to_string(),
        week_end: weekly_report.week_end().to_string(),
        total_duration: weekly_report.total_duration().to_string(),
        duration_per_category: weekly_report
            .duration_per_category()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        daily_durations_per_category,
    };

    (StatusCode::CREATED, Json(response))
}
