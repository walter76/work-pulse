use uuid::Uuid;

use crate::{
    adapters::{AccountingCategoriesListRepository, AccountingCategoriesListRepositoryError},
    entities::accounting::{AccountingCategory, AccountingCategoryId},
};

/// Represents a record for a `AccountingCategory`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AccountingCategoryRecord {
    /// The unique identifier for the record.
    id: Uuid,

    /// The name of the accounting category.
    name: String,
}

impl AccountingCategoryRecord {
    /// Converts a `AccountingCategory` entity to a `AccountingCategoryRecord`.
    ///
    /// # Arguments
    ///
    /// - `category`: The `AccountingCategory` entity to convert.
    fn from_entity(category: AccountingCategory) -> Self {
        Self {
            id: category.id().0,
            name: category.name().to_string(),
        }
    }

    /// Converts a `AccountingCategoryRecord` to a `AccountingCategory` entity.
    fn to_entity(&self) -> AccountingCategory {
        AccountingCategory::with_id(AccountingCategoryId(self.id), self.name.clone())
    }
}

/// In-memory implementation of a repository for accounting categories.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InMemoryPamCategoriesListRepository {
    categories: Vec<AccountingCategoryRecord>,
}

impl InMemoryPamCategoriesListRepository {
    /// Creates a new in-memory repository for accounting categories.
    pub fn new() -> Self {
        InMemoryPamCategoriesListRepository {
            categories: Vec::new(),
        }
    }
}

impl AccountingCategoriesListRepository for InMemoryPamCategoriesListRepository {
    fn get_all(&self) -> Vec<AccountingCategory> {
        self.categories
            .iter()
            .map(|record| record.to_entity())
            .collect()
    }

    fn get_by_id(&self, id: AccountingCategoryId) -> Option<AccountingCategory> {
        self.categories
            .iter()
            .find(|&record| record.id == id.0)
            .map(|record| record.to_entity())
    }

    fn add(&mut self, category: AccountingCategory) {
        let record = AccountingCategoryRecord::from_entity(category);
        self.categories.push(record);
    }

    fn update(
        &mut self,
        category: AccountingCategory,
    ) -> Result<(), AccountingCategoriesListRepositoryError> {
        if let Some(record) = self.categories.iter_mut().find(|r| r.id == category.id().0) {
            *record = AccountingCategoryRecord::from_entity(category);

            Ok(())
        } else {
            Err(AccountingCategoriesListRepositoryError::NotFound(
                category.id().clone(),
            ))
        }
    }

    fn delete(
        &mut self,
        id: AccountingCategoryId,
    ) -> Result<(), AccountingCategoriesListRepositoryError> {
        if let Some(pos) = self.categories.iter().position(|r| r.id == id.0) {
            self.categories.remove(pos);
            Ok(())
        } else {
            Err(AccountingCategoriesListRepositoryError::NotFound(id))
        }
    }

    fn get_or_create_by_name(
        &mut self,
        name: &str,
    ) -> Result<AccountingCategory, AccountingCategoriesListRepositoryError> {
        if let Some(record) = self.categories.iter().find(|r| r.name.eq(name)) {
            Ok(record.to_entity())
        } else {
            let new_category = AccountingCategory::new(name.to_string());
            self.add(new_category.clone());
            Ok(new_category)
        }
    }
}
