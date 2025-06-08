pub mod activities_list;
pub mod pam_categories_list;

use std::sync::{Arc, Mutex};

use crate::{adapters::{ActivitiesListRepository, PamCategoriesListRepository}, infra::repositories::in_memory::{activities_list::InMemoryActivitiesListRepository, pam_categories_list::InMemoryPamCategoriesListRepository}};

/// Represents a factory for creating in-memory repositories.
pub struct RepositoryFactory {
    pub activities_list_repository: Arc<Mutex<dyn ActivitiesListRepository>>,
    pub pam_categories_list_repository: Arc<Mutex<dyn PamCategoriesListRepository>>,
}

impl RepositoryFactory {
    /// Creates a new instance of `RepositoryFactory`.
    pub fn new() -> Self {
        let activities_list_repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let pam_categories_list_repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));

        Self {
            activities_list_repository,
            pam_categories_list_repository,
        }
    }
}
