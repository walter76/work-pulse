use axum::Router;
use cucumber::{World, given, then, when};
use tower::util::ServiceExt;
use work_pulse_service::services::health_check_service;
use work_pulse_service::services::health_check_service::HealthStatus;

#[derive(World)]
pub struct ServiceWorld {
    health_check_service: Option<Router>,
    health_status: Option<HealthStatus>,
    status_code: Option<u16>,
}

impl std::fmt::Debug for ServiceWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceWorld")
            .field(
                "health_check_service_initialized",
                &self.health_check_service.is_some(),
            )
            .finish()
    }
}

impl Default for ServiceWorld {
    fn default() -> Self {
        Self {
            health_check_service: None,
            health_status: None,
            status_code: None,
        }
    }
}

#[given("the health check service is running")]
async fn health_check_service_is_running(world: &mut ServiceWorld) {
    let router = health_check_service::router(None);
    world.health_check_service = Some(Router::new().nest("/api/v1/health", router.into()));
}

#[when(expr = "I send a GET request to {string}")]
async fn send_get_request(world: &mut ServiceWorld, path: String) {
    if let Some(router) = &world.health_check_service {
        let response = router
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .expect("Failed to send request");

        world.status_code = Some(response.status().as_u16());

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let health_status: HealthStatus = serde_json::from_slice(&body).unwrap();
        world.health_status = Some(health_status);
    } else {
        panic!("Health check service is not running");
    }
}

#[then(expr = "the response status code should be {int}")]
async fn check_response_status_code(world: &mut ServiceWorld, expected_status_code: u16) {
    if let Some(status_code) = world.status_code {
        assert_eq!(
            status_code, expected_status_code,
            "Expected status code {}, but got {}",
            expected_status_code, status_code
        );
    } else {
        panic!("No response received");
    }
}

#[then(expr = "the service should be healthy")]
async fn check_service_health(world: &mut ServiceWorld) {
    if let Some(health_status) = &world.health_status {
        assert!(
            health_status.status == "ok",
            "Expected service status to be 'ok', but got '{}'",
            health_status.status
        );
    } else {
        panic!("No response received");
    }
}

#[then(expr = "the database should be disabled")]
async fn check_database_disabled(world: &mut ServiceWorld) {
    if let Some(health_status) = &world.health_status {
        assert!(
            health_status.database == "disabled",
            "Expected database status to be 'disabled', but got '{}'",
            health_status.database
        );
    } else {
        panic!("No response received");
    }
}

#[tokio::main]
async fn main() {
    ServiceWorld::run("tests/features/health_check_service.feature").await;
}
