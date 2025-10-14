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
use work_pulse_core::{
    adapters::ActivitiesListRepository, infra::repositories::in_memory::RepositoryFactory,
    use_cases,
};

use crate::prelude::WEEKLY_REPORT_SERVICE_TAG;

/// A report summarizing activities for a specific week.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct WeeklyReport {
    /// The date (a sunday) when the week started in ISO 8601 format (YYYY-MM-DD).
    #[schema(example = "2025-10-12")]
    pub week_start: String,

    /// The total duration of all activities in the week in ISO 8601 format (PT1H).
    #[schema(example = "PT1800S")]
    pub total_duration: String,

    /// A map of accounting category IDs to total duration spent in that category in ISO 8601 format (PT1H).
    #[schema(example = r#"{"category-1": "PT3600S", "category-2": "PT7200S"}"#)]
    pub duration_per_category: HashMap<String, String>,
}

/// State for the Weekly Report Service.
struct WeeklyReportServiceState {
    /// The repository for accessing activities.
    pub activities_repository: Arc<std::sync::Mutex<dyn ActivitiesListRepository>>,
}

/// Type alias for a thread-safe, asynchronous mutex wrapping the service state.
type WeeklyReportStore = Mutex<WeeklyReportServiceState>;

/// Creates an OpenAPI router for the weekly report service.
///
/// # Arguments
///
/// - `repository_factory`: A reference to the `RepositoryFactory` used to create repositories.
///
/// # Returns
///
/// - An `OpenApiRouter` configured with routes and state for the weekly report service.
pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let store = Arc::new(Mutex::new(WeeklyReportServiceState {
        activities_repository: repository_factory.activities_list_repository.clone(),
    }));

    OpenApiRouter::new()
        .routes(routes!(generate_weekly_report))
        .with_state(store)
}

// Query parameters for generating weekly reports.
#[derive(Deserialize, IntoParams)]
struct GenerateWeeklyReportQuery {
    /// The date (a sunday) when the week started in ISO 8601 format (YYYY-MM-DD).
    #[param(example = "2025-10-12")]
    week_start_date: String,
}

/// Generates a weekly report for the specified week starting date.
///
/// # Arguments
///
/// - `State(store)`: The shared state containing the activities repository.
/// - `query`: The query parameters containing the week start date.
///
/// # Returns
///
/// - A tuple containing the HTTP status code and the generated weekly report in JSON format.
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
async fn generate_weekly_report(
    State(store): State<Arc<WeeklyReportStore>>,
    query: Query<GenerateWeeklyReportQuery>,
) -> impl IntoResponse {
    let week_start_date = query.week_start_date.parse().unwrap();
    let store = store.lock().await;
    let activities_repository = store.activities_repository.lock().unwrap();
    let weekly_report =
        use_cases::weekly_report::WeeklyReport::new(week_start_date, &*activities_repository);

    let response = WeeklyReport {
        week_start: weekly_report.week_start().to_string(),
        total_duration: weekly_report.total_duration().to_string(),
        duration_per_category: weekly_report
            .duration_per_category()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    };

    (StatusCode::CREATED, Json(response))
}
