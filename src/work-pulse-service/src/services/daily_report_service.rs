use std::sync::Arc;

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

use crate::prelude::DAILY_REPORT_SERVICE_TAG;

/// The Daily Report Activity.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct DailyReportActivity {
    /// The unique identifier for the activity.
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Option<String>,

    /// The duration of the activity in ISO 8601 format (PT1H).
    #[schema(example = "PT1800S")]
    duration: String,

    /// The start time of the activity in ISO 8601 format (HH:MM:SS).
    #[schema(example = "14:30:00")]
    start_time: String,

    /// The end time of the activity in ISO 8601 format (HH:MM:SS).
    #[schema(example = "15:30:00")]
    end_time: Option<String>,

    /// The accounting category ID associated with the activity.
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    accounting_category_id: String,

    /// The task itself.
    #[schema(example = "Code Review")]
    task: String,
}

/// The Daily Report.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct DailyReport {
    /// The date of the report.
    #[schema(example = "2023-01-10")]
    report_date: String,

    /// The total duration of activities for the day.
    #[schema(example = "PT1800S")]
    total_duration: String,

    /// The list of activities for the day.
    activities: Vec<DailyReportActivity>,
}

/// State for the Daily Report Service.
struct DailyReportServiceState {
    /// The repository for accessing activities.
    activities_repository: Arc<std::sync::Mutex<dyn ActivitiesListRepository>>,
}

/// Type alias for a thread-safe, asynchronous mutex wrapping the DailyReportServiceState.
type DailyReportStore = Mutex<DailyReportServiceState>;

/// Creates an OpenAPI router for the Daily Report Service.
///
/// # Arguments
///
/// - `repository_factory`: A reference to the `RepositoryFactory` used to create repositories.
///
/// # Returns
///
/// - An `OpenApiRouter` configured with routes for generating daily reports.
pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let store = Arc::new(Mutex::new(DailyReportServiceState {
        activities_repository: repository_factory.activities_list_repository.clone(),
    }));

    OpenApiRouter::new()
        .routes(routes!(generate_daily_report))
        .with_state(store)
}

/// Query parameters for generating daily reports.
#[derive(Deserialize, IntoParams)]
struct GenerateDailyReportQuery {
    /// The date of the activities being reported.
    report_date: String,
}

/// Generates a daily report for the specified date.
///
/// # Arguments
///
/// - `State(store)`: The shared state containing the activities repository.
/// - `query`: The query parameters containing the report date.
///
/// # Returns
///
/// - A response containing the generated daily report.
#[utoipa::path(
    get,
    path = "",
    tag = DAILY_REPORT_SERVICE_TAG,
    params(
        GenerateDailyReportQuery,
    ),
    responses(
        (status = 201, description = "Daily report created successfully", body = DailyReport)
    )
)]
async fn generate_daily_report(
    State(store): State<Arc<DailyReportStore>>,
    query: Query<GenerateDailyReportQuery>,
) -> impl IntoResponse {
    let report_date = query.report_date.parse().unwrap();
    let store = store.lock().await;
    let activities_repository = store.activities_repository.lock().unwrap();
    let daily_report =
        use_cases::daily_report::DailyReport::new(report_date, &*activities_repository);

    let activities: Vec<DailyReportActivity> = daily_report
        .activities()
        .iter()
        .map(|activity| DailyReportActivity {
            id: Some(activity.id().to_string()),
            duration: activity.duration().to_string(),
            start_time: activity.start_time().to_string(),
            end_time: activity.end_time().map(|t| t.to_string()),
            accounting_category_id: activity.accounting_category_id().to_string(),
            task: activity.task().to_string(),
        })
        .collect();
    let total_duration = daily_report.total_duration().to_string();

    let response = DailyReport {
        report_date: query.report_date.clone(),
        total_duration,
        activities,
    };

    (StatusCode::CREATED, Json(response))
}
