use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::{infra::repositories::RepositoryFactory, use_cases::pam_categories_list::PamCategoriesList};

type Store = Mutex<PamCategoriesList>;

/// The PAM Category.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct PamCategory {
    /// The unique identifier for the category.
    id: String,

    /// The name of the category.
    #[schema(example = "Current Version")]
    name: String,
}

pub(super) fn router(repository_factory: &RepositoryFactory) -> OpenApiRouter {
    let store = Arc::new(Mutex::new(PamCategoriesList::new(repository_factory.pam_categories_list_repository.clone())));

    OpenApiRouter::new()
        .routes(routes!(list_pam_categories))
        .with_state(store)
}

/// Lists all PAM categories.
#[utoipa::path(
    get,
    path = "",
    tag = PAM_CATEGORIES_SERVICE_TAG,
    responses(
        (status = 200, description = "List all PAM categories", body = [PamCategory])
    ),
    tag = "PAM Categories"
)]
async fn list_pam_categories(State(store): State<Arc<Store>>) -> Json<Vec<PamCategory>> {
    let pam_categories_list = store.lock().await;

    let categories = pam_categories_list
        .categories()
        .iter()
        .map(|category| PamCategory {
            id: category.id().to_string(),
            name: category.name().to_string(),
        }).collect::<Vec<_>>().into();

    Json(categories)
}
