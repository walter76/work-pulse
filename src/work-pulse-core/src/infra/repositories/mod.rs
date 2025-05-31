use uuid::Uuid;

use crate::{adapters::PamCategoriesListRepository, entities::pam::{PamCategory, PamCategoryId}};

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
}
