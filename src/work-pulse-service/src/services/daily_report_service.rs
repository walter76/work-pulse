use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{
    adapters::ActivitiesListRepository, infra::repositories::in_memory::RepositoryFactory, use_cases,
};

use crate::prelude::DAILY_REPORT_SERVICE_TAG;

/// The Daily Report Activity.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct DailyReportActivity {
    /// The unique identifier for the activity.
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

struct ActivitiesStore {
    pub repository: Arc<std::sync::Mutex<dyn ActivitiesListRepository>>,
}

type Store = Mutex<ActivitiesStore>;

pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let store = Arc::new(Mutex::new(ActivitiesStore {
        repository: repository_factory.activities_list_repository.clone(),
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
    State(store): State<Arc<Store>>,
    query: Query<GenerateDailyReportQuery>,
) -> impl IntoResponse {
    let report_date = query.report_date.parse().unwrap();
    let store = store.lock().await;
    let repository = store.repository.lock().unwrap();
    let daily_report = use_cases::daily_report::DailyReport::new(report_date, &*repository);

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
