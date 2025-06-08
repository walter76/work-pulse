use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{entities::{activity::ActivityId, pam::PamCategoryId}, infra::repositories::in_memory::RepositoryFactory, use_cases::activities_list::ActivitiesList};

use crate::prelude::ACTIVITIES_LIST_SERVICE_TAG;

type Store = Mutex<ActivitiesList>;

/// The Activity.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct Activity {
    /// The unique identifier for the activity.
    id: String,

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
            id: entity.id().to_string(),
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
            ActivityId::parse_str(self.id.as_str()).expect("Invalid ID format"),
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
    let store = Arc::new(Mutex::new(ActivitiesList::new(repository_factory.activities_list_repository.clone())));

    OpenApiRouter::new()
        .routes(routes!(list_activities, create_activity))
        .with_state(store)
}

/// Lists all activities.
#[utoipa::path(
    get,
    path = "",
    tag = ACTIVITIES_LIST_SERVICE_TAG,
    responses(
        (status = 200, description = "List all activities successfully", body = Vec<Activity>),
    )
)]
async fn list_activities(State(store): State<Arc<Store>>) -> Json<Vec<Activity>> {
    let activities_list = store.lock().await;

    let activities = activities_list
        .activities()
        .iter()
        .map(Activity::from_entity)
        .collect();

    Json(activities)
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
    State(store): State<Arc<Store>>,
    Json(new_activity): Json<Activity>,
) -> impl IntoResponse {
    let mut activities_list = store.lock().await;

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

        assert_eq!(activity.id, entity.id().to_string());
        assert_eq!(activity.date, "2023-01-10");
        assert_eq!(activity.start_time, "14:30:00");
        assert_eq!(activity.end_time, Some("15:30:00".to_string()));
        assert_eq!(activity.pam_category_id, entity.pam_category_id().to_string());
        assert_eq!(activity.task, "Test Task");
    }

    #[test]
    fn activity_to_entity_should_convert_correctly() {
        let activity = Activity {
            id: ActivityId::new().to_string(),
            date: "2023-01-10".to_string(),
            start_time: "14:30:00".to_string(),
            end_time: Some("15:30:00".to_string()),
            pam_category_id: PamCategoryId::new().to_string(),
            task: "Test Task".to_string(),
        };

        let entity = activity.to_entity();

        assert_eq!(entity.id().to_string(), activity.id);
        assert_eq!(entity.date().to_string(), "2023-01-10");
        assert_eq!(entity.start_time().to_string(), "14:30:00");
        assert_eq!(entity.end_time().map(|t| t.to_string()), Some("15:30:00".to_string()));
        assert_eq!(entity.pam_category_id().to_string(), activity.pam_category_id);
        assert_eq!(entity.task(), "Test Task");
    }
}