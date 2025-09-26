use std::sync::Arc;

use axum::{extract::{Multipart, Path, Query, State}, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{entities::{activity::ActivityId, pam::PamCategoryId}, infra::{importers::csv_activities_importer::CsvActivitiesImporter, repositories::in_memory::RepositoryFactory}, use_cases::activities_list::ActivitiesList};

use crate::prelude::ACTIVITIES_LIST_SERVICE_TAG;

type ActivitiesStore = Mutex<ActivitiesList>;

#[derive(Clone)]
struct ServiceState {
    activities_store: Arc<ActivitiesStore>,
    csv_activities_importer: Arc<Mutex<CsvActivitiesImporter>>,
}

/// The Activity.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct Activity {
    /// The unique identifier for the activity.
    id: Option<String>,

    /// The date when the activity was performed in ISO 8601 format (YYYY-MM-DD).
    #[schema(example = "2023-01-10")]
    date: String,

    /// The start time of the activity in ISO 8601 format (HH:MM:SS).
    #[schema(example = "14:30:00")]
    start_time: String,

    /// The end time of the activity in ISO 8601 format (HH:MM:SS).
    #[schema(example = "15:30:00")]
    end_time: Option<String>,

    /// The PAM category ID associated with the activity.
    pam_category_id: String,

    /// The task itself.
    #[schema(example = "Code Review")]
    task: String,
}

impl Activity {
    /// Converts a `work_pulse_core::entities::activity::Activity` entity to an `Activity` DTO.
    /// 
    /// # Arguments
    /// 
    /// - `entity`: A reference to the `work_pulse_core::entities::activity::Activity` entity.
    /// 
    /// # Returns
    /// 
    /// - An `Activity` DTO containing the data from the entity.
    fn from_entity(entity: &work_pulse_core::entities::activity::Activity) -> Self {
        Self {
            id: Some(entity.id().to_string()),
            date: entity.date().to_string(),
            start_time: entity.start_time().to_string(),
            end_time: entity.end_time().map(|t| t.to_string()),
            pam_category_id: entity.pam_category_id().to_string(),
            task: entity.task().to_string(),
        }
    }

    /// Converts the `Activity` DTO to a `work_pulse_core::entities::activity::Activity` entity.
    /// 
    /// # Returns
    /// 
    /// - A `work_pulse_core::entities::activity::Activity` entity constructed from the DTO.
    fn to_entity(&self) -> work_pulse_core::entities::activity::Activity {
        // TODO Handle potential parsing errors more gracefully

        let mut activity = work_pulse_core::entities::activity::Activity::with_id(
            ActivityId::parse_str(self.id.clone().unwrap().as_str()).expect("Invalid ID format"),
            self.date.parse().expect("Invalid date format"),
            self.start_time.parse().expect("Invalid start time format"),
            PamCategoryId::parse_str(self.pam_category_id.as_str()).expect("Invalid PAM category ID format"),
            self.task.clone(),
        );

        if let Some(end_time) = &self.end_time {
            activity.set_end_time(Some(end_time.parse().expect("Invalid end time format")));
        }

        activity
    }
}

pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let activities_store = Arc::new(Mutex::new(ActivitiesList::new(repository_factory.activities_list_repository.clone())));
    let csv_activities_importer = Arc::new(Mutex::new(CsvActivitiesImporter::new(repository_factory.pam_categories_list_repository.clone(), "2025".to_string())));

    let store = ServiceState {
        activities_store,
        csv_activities_importer,
    };

    OpenApiRouter::new()
        .routes(routes!(list_activities, create_activity))
        .routes(routes!(get_activity_by_id))
        .routes(routes!(update_pam_category))
        .routes(routes!(delete_activity))
        .routes(routes!(upload_activities_csv_raw, upload_activities_csv_multipart))
        .with_state(store)
}

/// Query parameters for listing activities.
#[derive(Deserialize, IntoParams)]
struct ListActivitiesQuery {
    /// The start date to filter activities by, in ISO 8601 format (YYYY-MM-DD).
    start_date: Option<String>,
    /// The end date to filter activities by, in ISO 8601 format (YYYY-MM-DD).
    end_date: Option<String>,
}

/// Lists all activities.
#[utoipa::path(
    get,
    path = "",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    params(
        ListActivitiesQuery,
    ),
    responses(
        (status = 200, description = "List all activities successfully", body = Vec<Activity>),
        (status = 400, description = "Invalid request - both start_date and end_date are required", body = String)
    )
)]
async fn list_activities(State(store): State<ServiceState>, query: Query<ListActivitiesQuery>,) -> impl IntoResponse {
    let activities_list = store.activities_store.lock().await;

    let activities = activities_list
        .activities()
        .iter()
        .map(Activity::from_entity)
        .collect::<Vec<_>>();

    // Filter activities by date rangeif provided
    match (&query.start_date, &query.end_date) {
        (Some(start_date), Some(end_date)) => {
            let filtered_activities = activities
                .into_iter()
                .filter(|activity| {
                    activity.date >= *start_date && activity.date <= *end_date
                })
                .collect::<Vec<_>>();
            Json(filtered_activities).into_response()
        }

        (Some(_), None) | (None, Some(_)) => {
            (
                StatusCode::BAD_REQUEST,
                Json("Both start_date and end_date are required when filtering by date.".to_string())
            ).into_response()
        }

        (None, None) => Json(activities).into_response(),
    }
}

/// Get an activity by ID.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    params(
        ("id" = String, Path, description = "The unique identifier of the activity to retrieve")
    ),
    responses(
        (status = 200, description = "Activity successfully retrieved", body = Activity),
        (status = 404, description = "Activity not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn get_activity_by_id(
    Path(id): Path<String>,
    State(store): State<ServiceState>,
) -> impl IntoResponse {
    let activities_list = store.activities_store.lock().await;

    match ActivityId::parse_str(&id) {
        Ok(activity_id) => match activities_list.get_by_id(&activity_id) {
            Some(activity) => (
                StatusCode::OK,
                Json(Activity::from_entity(&activity)),
            ).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json("Activity not found".to_string()),
            ).into_response(),
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json("Invalid activity ID format".to_string()),
        ).into_response(),
    }
}

/// Creates a new Activity.
#[utoipa::path(
    post,
    path = "",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    request_body = Activity,
    responses(
        (status = 201, description = "New Activity successfully created", body = Activity),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn create_activity(
    State(store): State<ServiceState>,
    Json(new_activity): Json<Activity>,
) -> impl IntoResponse {
    let mut activities_list = store.activities_store.lock().await;

    let date = new_activity.date.parse().expect("Invalid date format");
    let start_time = new_activity.start_time.parse().expect("Invalid start time format");
    let end_time = new_activity.end_time.as_ref().map(|t| t.parse().expect("Invalid end time format"));
    let pam_category_id = PamCategoryId::parse_str(new_activity.pam_category_id.as_str()).expect("Invalid PAM category ID format");

    let activity = activities_list.record(
        date,
        start_time,
        end_time,
        pam_category_id,
        new_activity.task.clone(),
    );

    (
        StatusCode::CREATED,
        Json(Activity::from_entity(&activity)),
    )
}

/// Updates an existing activity.
#[utoipa::path(
    put,
    path = "",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    request_body = Activity,
    responses(
        (status = 200, description = "Activity successfully updated", body = Activity),
        (status = 400, description = "Invalid request", body = String),
        (status = 404, description = "Activity not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn update_pam_category(
    State(store): State<ServiceState>,
    Json(updated_activity): Json<Activity>,
) -> impl IntoResponse {
    let mut activities_list = store.activities_store.lock().await;

    if updated_activity.id.is_none() {
        return (
                StatusCode::BAD_REQUEST,
                Json("An ID for Activity is required".to_string())
            ).into_response();
    }

    let updated_activity = updated_activity.to_entity();

    match activities_list.update(updated_activity.clone()) {
        Ok(_) => (
                StatusCode::OK,
                Json(Activity::from_entity(&updated_activity))
            ).into_response(),
        Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string())
            ).into_response(),
    }
}

/// Deletes an activity by ID.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    params(
        ("id" = String, Path, description = "The unique identifier of the activity to delete")
    ),
    responses(
        (status = 204, description = "Activity successfully deleted"),
        (status = 400, description = "Invalid request", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn delete_activity(
    Path(id): Path<String>,
    State(store): State<ServiceState>,
) -> impl IntoResponse {
    let mut activities_list = store.activities_store.lock().await;

    match ActivityId::parse_str(&id) {
        Ok(activity_id) => match activities_list.delete(activity_id) {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(err.to_string())
                ).into_response(),
        },
        Err(_) => (
                StatusCode::BAD_REQUEST,
                Json("Invalid activity ID format".to_string())
            ).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/upload-csv",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    request_body(content = String, content_type = "text/csv", description = "CSV file containing activities data"),
    responses(
        (status = 200, description = "CSV file processed successfully"),
        (status = 400, description = "Invalid CSV format", body = String)
    )
)]
async fn upload_activities_csv_raw(
    State(store): State<ServiceState>,
    body: String,
) -> impl IntoResponse {
    if body.is_empty() {
        (StatusCode::BAD_REQUEST, Json("No CSV data provided".to_string())).into_response()
    } else {
        let mut activities_list = store.activities_store.lock().await;

        let mut csv_importer = store.csv_activities_importer.lock().await;
        let reader = body.as_bytes();

        match activities_list.import(&mut *csv_importer, reader) {
            Ok(_) => (StatusCode::OK, Json("CSV file processed successfully".to_string())).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response(),
        }
    }
}

#[utoipa::path(
    post,
    path = "/upload-csv",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    request_body(content_type = "multipart/form-data", description = "CSV file upload"),
    responses(
        (status = 200, description = "CSV file processed successfully"),
        (status = 400, description = "Invalid CSV format", body = String)
    )
)]
async fn upload_activities_csv_multipart(
    State(store): State<ServiceState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut csv_content: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("file") {
            csv_content = Some(String::from_utf8_lossy(&field.bytes().await.unwrap_or_default()).to_string());
            break;
        }
    }

    if let Some(csv_content) = csv_content {
        let mut activities_list = store.activities_store.lock().await;

        let mut csv_importer = store.csv_activities_importer.lock().await;
        let reader = csv_content.as_bytes();

        match activities_list.import(&mut *csv_importer, reader) {
            Ok(_) => (StatusCode::OK, Json("CSV file processed successfully".to_string())).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, Json("No file field in multipart data".to_string())).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{NaiveDate, NaiveTime};
    use work_pulse_core::entities::{activity::ActivityId, pam::PamCategoryId};

    #[test]
    fn activity_from_entity_should_convert_correctly() {
        let mut entity = work_pulse_core::entities::activity::Activity::with_id(
            ActivityId::new(),
            NaiveDate::from_ymd_opt(2023, 1, 10).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            PamCategoryId::new(),
            "Test Task".to_string(),
        );
        entity.set_end_time(Some(NaiveTime::from_hms_opt(15, 30, 0).unwrap()));

        let activity = Activity::from_entity(&entity);

        assert_eq!(activity.id.unwrap(), entity.id().to_string());
        assert_eq!(activity.date, "2023-01-10");
        assert_eq!(activity.start_time, "14:30:00");
        assert_eq!(activity.end_time, Some("15:30:00".to_string()));
        assert_eq!(activity.pam_category_id, entity.pam_category_id().to_string());
        assert_eq!(activity.task, "Test Task");
    }

    #[test]
    fn activity_to_entity_should_convert_correctly() {
        let activity = Activity {
            id: Some(ActivityId::new().to_string()),
            date: "2023-01-10".to_string(),
            start_time: "14:30:00".to_string(),
            end_time: Some("15:30:00".to_string()),
            pam_category_id: PamCategoryId::new().to_string(),
            task: "Test Task".to_string(),
        };

        let entity = activity.to_entity();

        assert_eq!(entity.id().to_string(), activity.id.unwrap());
        assert_eq!(entity.date().to_string(), "2023-01-10");
        assert_eq!(entity.start_time().to_string(), "14:30:00");
        assert_eq!(entity.end_time().map(|t| t.to_string()), Some("15:30:00".to_string()));
        assert_eq!(entity.pam_category_id().to_string(), activity.pam_category_id);
        assert_eq!(entity.task(), "Test Task");
    }
}