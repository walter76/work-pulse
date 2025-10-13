mod services;

use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use services::accounting_categories_service;
use work_pulse_core::infra::repositories::in_memory::RepositoryFactory;

use crate::services::activities_list_service;

mod prelude {
    pub const ACTIVITIES_LIST_SERVICE_TAG: &str = "activities-list-service";
    pub const ACCOUNTING_CATEGORIES_SERVICE_TAG: &str = "accounting-categories-service";
    pub const DAILY_REPORT_SERVICE_TAG: &str = "daily-report-service";
    pub const WEEKLY_REPORT_SERVICE_TAG: &str = "weekly-report-service";
}

// TODO Implement health check service

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = prelude::ACTIVITIES_LIST_SERVICE_TAG, description = "Activities List Service"),
            (name = prelude::ACCOUNTING_CATEGORIES_SERVICE_TAG, description = "Accounting Categories Service"),
            (name = prelude::DAILY_REPORT_SERVICE_TAG, description = "Daily Report Service"),
            (name = prelude::WEEKLY_REPORT_SERVICE_TAG, description = "Weekly Report Service"),
        )
    )]
    struct ApiDoc;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let repository_factory = RepositoryFactory::new();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(
            "/api/v1/accounting-categories",
            accounting_categories_service::router(&repository_factory),
        )
        .nest(
            "/api/v1/activities",
            activities_list_service::router(&repository_factory),
        )
        .nest(
            "/api/v1/daily-report",
            services::daily_report_service::router(&repository_factory),
        )
        .nest(
            "/api/v1/weekly-report",
            services::weekly_report_service::router(&repository_factory),
        )
        .split_for_parts();

    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    // configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = router.layer(cors).layer(TraceLayer::new_for_http());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    tracing::info!("Starting server at http://{}", address);
    tracing::info!(
        "OpenAPI documentation available at: http://{}/swagger-ui",
        address
    );

    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}
