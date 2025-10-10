pub mod activities_list;
pub mod accounting_categories_list;

use std::sync::{Arc, Mutex};

use crate::{
    adapters::{AccountingCategoriesListRepository, ActivitiesListRepository},
    infra::repositories::in_memory::{
        activities_list::InMemoryActivitiesListRepository,
        accounting_categories_list::InMemoryAccountingCategoriesListRepository,
    },
};

/// Represents a factory for creating in-memory repositories.
pub struct RepositoryFactory {
    pub activities_list_repository: Arc<Mutex<dyn ActivitiesListRepository>>,
    pub accounting_categories_list_repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
}

impl RepositoryFactory {
    /// Creates a new instance of `RepositoryFactory`.
    pub fn new() -> Self {
        let activities_list_repository =
            Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let accounting_categories_list_repository =
            Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));

        Self {
            activities_list_repository,
            accounting_categories_list_repository,
        }
    }
}
