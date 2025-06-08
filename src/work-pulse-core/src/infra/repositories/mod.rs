pub mod in_memory;

use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{adapters::{PamCategoriesListRepository, PamCategoriesListRepositoryError}, entities::pam::{PamCategory, PamCategoryId}};

/// Represents a record for a `PamCategory`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PamCategoryRecord {
    /// The unique identifier for the record.
    id: Uuid,

    /// The name of the PAM category.
    name: String,
}

impl PamCategoryRecord {
    /// Converts a `PamCategory` entity to a `PamCategoryRecord`.
    /// 
    /// # Arguments
    /// 
    /// - `category`: The `PamCategory` entity to convert.
    fn from_entity(category: PamCategory) -> Self {
        PamCategoryRecord {
            id: category.id().0,
            name: category.name().to_string(),
        }
    }

    /// Converts a `PamCategoryRecord` to a `PamCategory` entity.
    fn to_entity(&self) -> PamCategory {
        PamCategory::with_id(PamCategoryId(self.id), self.name.clone())
    }
}


/// In-memory implementation of a repository for PAM categories.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InMemoryPamCategoriesListRepository {
    categories: Vec<PamCategoryRecord>,
}

impl InMemoryPamCategoriesListRepository {
    /// Creates a new in-memory repository for PAM categories.
    pub fn new() -> Self {
        InMemoryPamCategoriesListRepository {
            categories: Vec::new(),
        }
    }
}

impl PamCategoriesListRepository for InMemoryPamCategoriesListRepository {
    fn get_all(&self) -> Vec<PamCategory> {
        self.categories.iter()
            .map(|record| record.to_entity()).collect()
    }

    fn get_by_id(&self, id: PamCategoryId) -> Option<PamCategory> {
        self.categories.iter()
            .find(|&record| record.id == id.0)
            .map(|record| record.to_entity())
    }

    fn add(&mut self, category: PamCategory) {
        let record = PamCategoryRecord::from_entity(category);
        self.categories.push(record);
    }

    fn update(&mut self, category: PamCategory) -> Result<(), PamCategoriesListRepositoryError> {
        if let Some(record) = self.categories.iter_mut().find(|r| r.id == category.id().0) {
            *record = PamCategoryRecord::from_entity(category);

            Ok(())
        } else {
            Err(PamCategoriesListRepositoryError::NotFound(category.id().clone()))
        }
    }

    fn delete(&mut self, id: PamCategoryId) -> Result<(), PamCategoriesListRepositoryError> {
        if let Some(pos) = self.categories.iter().position(|r| r.id == id.0) {
            self.categories.remove(pos);
            Ok(())
        } else {
            Err(PamCategoriesListRepositoryError::NotFound(id))
        }
    }
}

pub struct RepositoryFactory {
    pub pam_categories_list_repository: Arc<Mutex<dyn PamCategoriesListRepository>>,
}

impl RepositoryFactory {
    pub fn new() -> Self {
        let pam_categories_list_repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));

        Self {
            pam_categories_list_repository,
        }
    }
}
