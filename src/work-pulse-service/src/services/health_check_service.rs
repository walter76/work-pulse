use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::infra::repositories::postgres::PsqlConnection;

use crate::prelude::HEALTH_CHECK_SERVICE_TAG;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
}

pub fn router(connection: Option<Arc<PsqlConnection>>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(health_check))
        .with_state(connection)
}

#[utoipa::path(
    get,
    path = "",
    tag = HEALTH_CHECK_SERVICE_TAG,
    responses(
        (status = 200, description = "Service is healthy", body = HealthStatus),
        (status = 503, description = "Service is unhealthy", body = HealthStatus)
    )
)]
async fn health_check(State(connection): State<Option<Arc<PsqlConnection>>>) -> impl IntoResponse {
    if connection.is_none() {
        return (
            StatusCode::OK,
            Json(HealthStatus {
                status: "ok".to_string(),
                database: "disabled".to_string(),
            }),
        )
            .into_response();
    }

    match connection.as_ref().unwrap().ping().await {
        Ok(()) => (
            StatusCode::OK,
            Json(HealthStatus {
                status: "ok".to_string(),
                database: "connected".to_string(),
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthStatus {
                status: "error".to_string(),
                database: "disconnected".to_string(),
            }),
        )
            .into_response(),
    }
}
