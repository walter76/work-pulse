mod services;

use clap::Parser;
use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;
use work_pulse_core::adapters::{AccountingCategoriesListRepository, ActivitiesListRepository};

use work_pulse_core::infra::repositories::{
    in_memory::{
        accounting_categories_list::InMemoryAccountingCategoriesListRepository,
        activities_list::InMemoryActivitiesListRepository,
    },
    postgres::{
        PsqlConnection, accounting_categories_list::PsqlAccountingCategoriesListRepository,
        activities_list::PsqlActivitiesListRepository,
    },
};

mod prelude {
    pub const ACTIVITIES_LIST_SERVICE_TAG: &str = "activities-list-service";
    pub const ACCOUNTING_CATEGORIES_SERVICE_TAG: &str = "accounting-categories-service";
    pub const DAILY_REPORT_SERVICE_TAG: &str = "daily-report-service";
    pub const WEEKLY_REPORT_SERVICE_TAG: &str = "weekly-report-service";
}

const CONNECTION_STRING: &str = "postgres://workpulse:supersecret@localhost:5432/workpulse";

// TODO Implement health check service

#[derive(clap::Parser)]
#[command(
    name = "Work Pulse API",
    version = "0.1",
    author = "Walter Stocker <wrstocke@googlemail.com>",
    about = "Work Pulse API Server"
)]
struct Cli {
    /// Use in-memory repositories instead of PostgreSQL
    #[arg(long, default_value_t = false)]
    use_in_memory_repositories: bool,
}

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

    let cli = Cli::parse();
    let (router, api) = if cli.use_in_memory_repositories {
        let (accounting_categories_repository, activities_list_repository) =
            create_in_memory_repositories().await;

        create_open_api_router(accounting_categories_repository, activities_list_repository)
            .split_for_parts()
    } else {
        let (accounting_categories_repository, activities_list_repository) =
            create_psql_repositories().await;

        create_open_api_router(accounting_categories_repository, activities_list_repository)
            .split_for_parts()
    };

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

/// Creates PostgreSQL repositories for production use.
///
/// Returns a tuple containing:
/// - An `Arc<Mutex<PsqlAccountingCategoriesListRepository>>`
/// - An `Arc<Mutex<PsqlActivitiesListRepository>>`
async fn create_psql_repositories() -> (
    Arc<Mutex<PsqlAccountingCategoriesListRepository>>,
    Arc<Mutex<PsqlActivitiesListRepository>>,
) {
    let psql_connection = PsqlConnection::with_database_url(CONNECTION_STRING).await;
    let psql_accounting_categories_repository = Arc::new(Mutex::new(
        PsqlAccountingCategoriesListRepository::new(psql_connection.clone()),
    ));
    let psql_activities_list_repository = Arc::new(Mutex::new(PsqlActivitiesListRepository::new(
        psql_connection.clone(),
    )));

    (
        psql_accounting_categories_repository,
        psql_activities_list_repository,
    )
}

/// Creates in-memory repositories for testing purposes.
///
/// Returns a tuple containing:
/// - An `Arc<Mutex<InMemoryAccountingCategoriesListRepository>>`
/// - An `Arc<Mutex<InMemoryActivitiesListRepository>>`
async fn create_in_memory_repositories() -> (
    Arc<Mutex<InMemoryAccountingCategoriesListRepository>>,
    Arc<Mutex<InMemoryActivitiesListRepository>>,
) {
    let in_memory_accounting_categories_repository =
        Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
    let in_memory_activities_list_repository =
        Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));

    (
        in_memory_accounting_categories_repository,
        in_memory_activities_list_repository,
    )
}

/// Creates an OpenAPI router with the provided repositories.
///
/// # Arguments
///
/// - `accounting_categories_repository`: An `Arc<Mutex<R>>` where `R` implements `AccountingCategoriesListRepository`.
/// - `activities_list_repository`: An `Arc<Mutex<T>>` where `T` implements `ActivitiesListRepository`.
///
/// # Returns
///
/// An `OpenApiRouter` configured with the provided repositories.
fn create_open_api_router<R, T>(
    accounting_categories_repository: Arc<Mutex<R>>,
    activities_list_repository: Arc<Mutex<T>>,
) -> OpenApiRouter
where
    R: AccountingCategoriesListRepository + Send + Sync + 'static,
    T: ActivitiesListRepository + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .nest(
            "/api/v1/accounting-categories",
            services::accounting_categories_service::router(
                accounting_categories_repository.clone(),
            ),
        )
        .nest(
            "/api/v1/activities",
            services::activities_list_service::router(
                activities_list_repository.clone(),
                accounting_categories_repository.clone(),
            ),
        )
        .nest(
            "/api/v1/daily-report",
            services::daily_report_service::router(activities_list_repository.clone()),
        )
        .nest(
            "/api/v1/weekly-report",
            services::weekly_report_service::router(activities_list_repository.clone()),
        )
}
