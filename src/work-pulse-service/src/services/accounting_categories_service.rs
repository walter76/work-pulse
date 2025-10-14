use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{
    entities::accounting::AccountingCategoryId, infra::repositories::in_memory::RepositoryFactory,
    use_cases::accounting_categories_list::AccountingCategoriesList,
};

use crate::prelude::ACCOUNTING_CATEGORIES_SERVICE_TAG;

/// Type alias for a thread-safe, asynchronous mutex wrapping an `AccountingCategoriesList`.
type AccountingCategoriesListStore = Mutex<AccountingCategoriesList>;

/// The Accounting Category.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct AccountingCategory {
    /// The unique identifier for the category.
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Option<String>,

    /// The name of the category.
    #[schema(example = "Current Version")]
    name: String,
}

impl AccountingCategory {
    /// Converts a `work_pulse_core::entities::accounting::AccountingCategory` entity to a `AccountingCategory` DTO.
    ///
    /// # Arguments
    ///
    /// - `entity`: A reference to the `work_pulse_core::entities::accounting::AccountingCategory` entity.
    ///
    /// # Returns
    ///
    /// - A `AccountingCategory` DTO containing the data from the entity.
    fn from_entity(entity: &work_pulse_core::entities::accounting::AccountingCategory) -> Self {
        Self {
            id: Some(entity.id().to_string()),
            name: entity.name().to_string(),
        }
    }
}

/// Creates an OpenAPI router for accounting categories service.
///
/// # Arguments
///
/// - `repository_factory`: A reference to the `RepositoryFactory` used to create repositories.
///
/// # Returns
///
/// - An `OpenApiRouter` configured with routes for managing accounting categories.
pub fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let store = Arc::new(Mutex::new(AccountingCategoriesList::new(
        repository_factory
            .accounting_categories_list_repository
            .clone(),
    )));

    OpenApiRouter::new()
        .routes(routes!(
            list_accounting_categories,
            create_accounting_category
        ))
        .routes(routes!(update_accounting_category))
        .routes(routes!(delete_accounting_category))
        .with_state(store)
}

/// Lists all accounting categories.
#[utoipa::path(
    get,
    path = "",
    tag = ACCOUNTING_CATEGORIES_SERVICE_TAG,
    responses(
        (status = 200, description = "List all accounting categories successfully", body = [AccountingCategory])
    )
)]
async fn list_accounting_categories(
    State(store): State<Arc<AccountingCategoriesListStore>>,
) -> Json<Vec<AccountingCategory>> {
    let accounting_categories_list = store.lock().await;

    let categories = accounting_categories_list
        .categories()
        .iter()
        .map(AccountingCategory::from_entity)
        .collect();

    Json(categories)
}

/// Creates a new accounting category.
#[utoipa::path(
    post,
    path = "",
    tag = ACCOUNTING_CATEGORIES_SERVICE_TAG,
    request_body = AccountingCategory,
    responses(
        (status = 201, description = "New accounting category successfully created", body = AccountingCategory),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn create_accounting_category(
    State(store): State<Arc<AccountingCategoriesListStore>>,
    Json(new_category): Json<AccountingCategory>,
) -> impl IntoResponse {
    let mut accounting_categories_list = store.lock().await;

    match accounting_categories_list.create(new_category.name.as_str()) {
        Ok(accounting_category) => (
            StatusCode::CREATED,
            Json(AccountingCategory::from_entity(&accounting_category)),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

/// Updates an existing accounting category.
#[utoipa::path(
    put,
    path = "",
    tag = ACCOUNTING_CATEGORIES_SERVICE_TAG,
    request_body = AccountingCategory,
    responses(
        (status = 200, description = "Accounting category successfully updated", body = AccountingCategory),
        (status = 400, description = "Invalid request", body = String),
        (status = 404, description = "Accounting category not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn update_accounting_category(
    State(store): State<Arc<AccountingCategoriesListStore>>,
    Json(updated_category): Json<AccountingCategory>,
) -> impl IntoResponse {
    let mut accounting_categories_list = store.lock().await;

    if updated_category.id.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json("Category ID is required".to_string()),
        )
            .into_response();
    }

    let category_id = match AccountingCategoryId::parse_str(&updated_category.id.unwrap()) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("Invalid category ID format".to_string()),
            )
                .into_response();
        }
    };

    let updated_category = work_pulse_core::entities::accounting::AccountingCategory::with_id(
        category_id,
        updated_category.name.clone(),
    );

    match accounting_categories_list.update(updated_category.clone()) {
        Ok(_) => (
            StatusCode::OK,
            Json(AccountingCategory::from_entity(&updated_category)),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

/// Deletes an accounting category by ID.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = ACCOUNTING_CATEGORIES_SERVICE_TAG,
    params(
        ("id" = String, Path, description = "The unique identifier of the accounting category to delete")
    ),
    responses(
        (status = 204, description = "Accounting category successfully deleted"),
        (status = 400, description = "Invalid request", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
async fn delete_accounting_category(
    Path(id): Path<String>,
    State(store): State<Arc<AccountingCategoriesListStore>>,
) -> impl IntoResponse {
    let mut accounting_categories_list = store.lock().await;

    match AccountingCategoryId::parse_str(&id) {
        Ok(category_id) => match accounting_categories_list.delete(category_id) {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json("Invalid category ID format".to_string()),
        )
            .into_response(),
    }
}
