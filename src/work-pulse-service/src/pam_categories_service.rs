use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

/// The PAM Category.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct PamCategory {
    /// The unique identifier for the category.
    id: String,

    /// The name of the category.
    #[schema(example = "Current Version")]
    name: String,
}

pub(super) fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![list_pam_categories])
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
async fn list_pam_categories() -> Json<Vec<PamCategory>> {
    let categories = vec![
        PamCategory {
            id: "1".to_string(),
            name: "Current Version".to_string(),
        },
        PamCategory {
            id: "2".to_string(),
            name: "Previous Version".to_string(),
        },
    ];

    Json(categories)
}
