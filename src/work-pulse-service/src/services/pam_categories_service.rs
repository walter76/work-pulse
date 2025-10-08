use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{entities::accounting::PamCategoryId, infra::repositories::in_memory::RepositoryFactory, use_cases::pam_categories_list::PamCategoriesList};

use crate::prelude::PAM_CATEGORIES_SERVICE_TAG;

type Store = Mutex<PamCategoriesList>;

/// The PAM Category.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct PamCategory {
    /// The unique identifier for the category.
    id: Option<String>,

    /// The name of the category.
    #[schema(example = "Current Version")]
    name: String,
}

impl PamCategory {
    /// Converts a `work_pulse_core::entities::pam::PamCategory` entity to a `PamCategory` DTO.
    /// 
    /// # Arguments
    /// 
    /// - `entity`: A reference to the `work_pulse_core::entities::pam::PamCategory` entity.
    /// 
    /// # Returns
    /// 
    /// - A `PamCategory` DTO containing the data from the entity.
    fn from_entity(entity: &work_pulse_core::entities::accounting::PamCategory) -> Self {
        Self {
            id: Some(entity.id().to_string()),
            name: entity.name().to_string(),
        }
    }
}

pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    // FIXME Remove this temporary generation of test data
    let store = Arc::new(Mutex::new(PamCategoriesList::with_test_data(repository_factory.pam_categories_list_repository.clone())));

    OpenApiRouter::new()
        .routes(routes!(list_pam_categories, create_pam_category))
        .routes(routes!(update_pam_category))
        .routes(routes!(delete_pam_category))
        .with_state(store)
}

/// Lists all PAM categories.
#[utoipa::path(
    get,
    path = "",
    tag = PAM_CATEGORIES_SERVICE_TAG,
    responses(
        (status = 200, description = "List all PAM categories successfully", body = [PamCategory])
    )
)]
async fn list_pam_categories(State(store): State<Arc<Store>>) -> Json<Vec<PamCategory>> {
    let pam_categories_list = store.lock().await;

    let categories = pam_categories_list
        .categories()
        .iter()
        .map(PamCategory::from_entity)
        .collect();

    Json(categories)
}

/// Creates a new PAM category.
#[utoipa::path(
    post,
    path = "",
    tag = PAM_CATEGORIES_SERVICE_TAG,
    request_body = PamCategory,
    responses(
        (status = 201, description = "New PAM category successfully created", body = PamCategory),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn create_pam_category(
    State(store): State<Arc<Store>>,
    Json(new_category): Json<PamCategory>,
) -> impl IntoResponse {
    let mut pam_categories_list = store.lock().await;

    match pam_categories_list.create(new_category.name.as_str()) {
        Ok(pam_category) => (
                StatusCode::CREATED,
                Json(PamCategory::from_entity(&pam_category))
            ).into_response(),
        Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string())
            ).into_response()
        
    }
}

/// Updates an existing PAM category.
#[utoipa::path(
    put,
    path = "",
    tag = PAM_CATEGORIES_SERVICE_TAG,
    request_body = PamCategory,
    responses(
        (status = 200, description = "PAM category successfully updated", body = PamCategory),
        (status = 400, description = "Invalid request", body = String),
        (status = 404, description = "PAM category not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn update_pam_category(
    State(store): State<Arc<Store>>,
    Json(updated_category): Json<PamCategory>,
) -> impl IntoResponse {
    let mut pam_categories_list = store.lock().await;

    if updated_category.id.is_none() {
        return (
                StatusCode::BAD_REQUEST,
                Json("Category ID is required".to_string())
            ).into_response();
    }

    let category_id = match PamCategoryId::parse_str(&updated_category.id.unwrap()) {
        Ok(id) => id,
        Err(_) => return (
                StatusCode::BAD_REQUEST,
                Json("Invalid category ID format".to_string())
            ).into_response(),
    };

    let updated_category = work_pulse_core::entities::accounting::PamCategory::with_id(category_id, updated_category.name.clone());

    match pam_categories_list.update(updated_category.clone()) {
        Ok(_) => (
                StatusCode::OK,
                Json(PamCategory::from_entity(&updated_category))
            ).into_response(),
        Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string())
            ).into_response(),
    }
}

/// Deletes a PAM category by ID.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = PAM_CATEGORIES_SERVICE_TAG,
    params(
        ("id" = String, Path, description = "The unique identifier of the PAM category to delete")
    ),
    responses(
        (status = 204, description = "PAM category successfully deleted"),
        (status = 400, description = "Invalid request", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn delete_pam_category(
    Path(id): Path<String>,
    State(store): State<Arc<Store>>,
) -> impl IntoResponse {
    let mut pam_categories_list = store.lock().await;

    match PamCategoryId::parse_str(&id) {
        Ok(category_id) => match pam_categories_list.delete(category_id) {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(err.to_string())
                ).into_response(),
        },
        Err(_) => (
                StatusCode::BAD_REQUEST,
                Json("Invalid category ID format".to_string())
            ).into_response(),
    }
}
