use std::sync::{Arc, Mutex};

use sqlx::PgPool;

use crate::{
    adapters::AccountingCategoriesListRepository,
    infra::repositories::postgres::accounting_categories_list::PsqlAccountingCategoriesListRepository,
};

pub mod accounting_categories_list;

/// Represents a factory for creating Postgres repositories.
pub struct RepositoryFactory {
    /// Repository for managing accounting categories.
    pub accounting_categories_list_repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
}

impl RepositoryFactory {
    /// Creates a new instance of `RepositoryFactory`.
    pub fn new(connection_pool: PgPool) -> Self {
        let accounting_categories_list_repository =
            Arc::new(Mutex::new(PsqlAccountingCategoriesListRepository::new(connection_pool)));

        Self {
            accounting_categories_list_repository,
        }
    }
}
