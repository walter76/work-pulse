use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use work_pulse_core::infra::repositories::postgres::PsqlConnection;

pub const HEALTH_CHECK_SERVICE_TAG: &str = "health-check-service";

#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct HealthStatus {
    status: String,
    database: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn health_check_returns_200_when_no_database_configured() {
        let app = router(None);
        let app = axum::Router::new()
            .nest("/api/v1/health", app.into())
            .into_service();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let health: HealthStatus = serde_json::from_slice(&body).unwrap();

        assert_eq!(health.status, "ok");
        assert_eq!(health.database, "disabled");
    }

    #[tokio::test]
    #[ignore]
    async fn health_check_returns_200_when_database_connected() {
        let connection =
            PsqlConnection::with_database_url("postgresql://localhost:5432/test").await;
        let app = router(Some(Arc::new(connection)));
        let app = axum::Router::new()
            .nest("/api/v1/health", app.into())
            .into_service();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let health: HealthStatus = serde_json::from_slice(&body).unwrap();

        assert_eq!(health.status, "ok");
        assert_eq!(health.database, "connected");
    }

    #[tokio::test]
    #[ignore]
    async fn health_check_returns_503_when_database_disconnected() {
        let pool = work_pulse_core::infra::repositories::postgres::PsqlConnection::connect_lazy(
            "postgresql://invalid:5432/unreachable",
        );
        let connection = pool;
        let app = router(Some(Arc::new(connection)));
        let app = axum::Router::new()
            .nest("/api/v1/health", app.into())
            .into_service();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let health: HealthStatus = serde_json::from_slice(&body).unwrap();

        assert_eq!(health.status, "error");
        assert_eq!(health.database, "disconnected");
    }
}
