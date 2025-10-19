use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::{
    adapters::AccountingCategoriesListRepository,
    infra::repositories::postgres::accounting_categories_list::PsqlAccountingCategoriesListRepository,
};

pub mod accounting_categories_list;

/// Error type for the repository factory.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum RepositoryFactoryError {
    /// Error indicating a database-related issue.
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Represents a factory for creating Postgres repositories.
pub struct RepositoryFactory {
    /// Repository for managing accounting categories.
    pub accounting_categories_list_repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
}

impl RepositoryFactory {
    /// Creates a new instance of `RepositoryFactory`.
    pub fn new() -> Self {
        let accounting_categories_list_repository =
            Arc::new(Mutex::new(PsqlAccountingCategoriesListRepository::new()));

        Self {
            accounting_categories_list_repository,
        }
    }
}
