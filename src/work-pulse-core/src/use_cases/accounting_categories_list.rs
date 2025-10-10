use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::{
    adapters::AccountingCategoriesListRepository,
    entities::accounting::{AccountingCategory, AccountingCategoryId},
};

/// Represents an error that can occur while managing accounting categories.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum AccountingCategoriesListError {
    /// An accounting category with the specified name already exists.
    #[error("An accounting category with the name '{0}' already exists.")]
    AccountingCategoryAlreadyExists(String),

    /// An accounting category with the ID does not exists.
    #[error("Accounting category with the ID `{0}` does not exists.")]
    NotFound(AccountingCategoryId),
}

/// Represents a list of all accounting categories.
pub struct AccountingCategoriesList {
    /// The list of accounting categories.
    repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
}

impl AccountingCategoriesList {
    /// Creates a new `AccountingCategoriesList`.
    ///
    /// # Arguments
    ///
    /// - `repository`: An `Arc<Mutex<dyn AccountingCategoriesListRepository>>` that provides access to the accounting categories repository.
    pub fn new(repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>) -> Self {
        Self { repository }
    }

    /// Creates a new `AccountingCategoriesList` with test data.
    ///
    /// # Arguments
    ///
    /// - `repository`: An `Arc<Mutex<dyn AccountingCategoriesListRepository>>` that provides access to the accounting categories repository.
    pub fn with_test_data(repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>) -> Self {
        // FIXME Remove this test data creation
        let mut accounting_categories_list = Self { repository };

        accounting_categories_list.create("Current Version").unwrap();
        accounting_categories_list.create("SWA Trainer").unwrap();
        accounting_categories_list.create("Techno Cluster").unwrap();
        accounting_categories_list.create("Other").unwrap();

        accounting_categories_list
    }

    /// Adds an accounting category to the list.
    ///
    /// # Arguments
    ///
    /// - `category_name`: The name of the accounting category to add.
    ///
    /// # Returns
    ///
    /// - `Ok(AccountingCategory)`: If the category was successfully created.
    /// - `Err(AccountingCategoriesListError)`: If a category with the same name already exists.
    pub fn create(
        &mut self,
        category_name: &str,
    ) -> Result<AccountingCategory, AccountingCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        // Check if a category with the same name already exists.
        if repo
            .get_all()
            .iter()
            .find(|category| category.name() == category_name)
            .is_some()
        {
            return Err(AccountingCategoriesListError::AccountingCategoryAlreadyExists(
                category_name.to_string(),
            ));
        }

        let accounting_category = AccountingCategory::new(category_name.to_string());
        repo.add(accounting_category.clone());

        Ok(accounting_category)
    }

    /// Returns the list of accounting categories.
    ///
    /// # Returns
    ///
    /// - `Vec<AccountingCategory>`: A vector containing all accounting categories.
    pub fn categories(&self) -> Vec<AccountingCategory> {
        let repo = self.repository.lock().unwrap();
        repo.get_all()
    }

    /// Updates an existing accounting category in the list.
    ///
    /// # Arguments
    ///
    /// - `category`: The `AccountingCategory` instance with updated information to be saved in the repository.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the category was successfully updated.
    /// - `Err(AccountingCategoriesListError)`: If the category with the specified ID does not exist.
    pub fn update(&mut self, category: AccountingCategory) -> Result<(), AccountingCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        let category_id = category.id().clone();
        repo.update(category)
            .map_err(|_| AccountingCategoriesListError::NotFound(category_id))
    }

    /// Deletes an accounting category from the list.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier of the accounting category to be deleted.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the category was successfully deleted.
    /// - `Err(AccountingCategoriesListError)`: If the category with the specified ID does not exist.
    pub fn delete(&mut self, id: AccountingCategoryId) -> Result<(), AccountingCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        repo.delete(id.clone())
            .map_err(|_| AccountingCategoriesListError::NotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repositories::in_memory::accounting_categories_list::InMemoryAccountingCategoriesListRepository;

    use super::*;

    #[test]
    fn accounting_categories_list_create_should_add_category() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).unwrap();

        assert_eq!(categories_list.categories().len(), 1);
        assert_eq!(categories_list.categories()[0].name(), category_name);
    }

    #[test]
    fn accounting_categories_list_create_should_fail_when_category_exists() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).unwrap();

        let result = categories_list.create(category_name);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountingCategoriesListError::AccountingCategoryAlreadyExists(category_name.to_string())
        );
    }

    #[test]
    fn accounting_categories_list_should_return_empty_when_no_categories() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let categories_list = AccountingCategoriesList::new(repository);

        assert!(categories_list.categories().is_empty());
    }

    #[test]
    fn accounting_categories_list_should_return_all_categories() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        categories_list.create("Category 1").unwrap();
        categories_list.create("Category 2").unwrap();

        let categories = categories_list.categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name(), "Category 1");
        assert_eq!(categories[1].name(), "Category 2");
    }

    #[test]
    fn accounting_categories_list_update_should_modify_existing_category() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Original Category";
        let mut category = categories_list.create(category_name).unwrap();

        let updated_name = "Updated Category";
        category.set_name(updated_name.to_string());

        categories_list.update(category).unwrap();

        let categories = categories_list.categories();
        let actual_name = categories.first().map(|c| c.name()).unwrap();
        assert_eq!(actual_name, updated_name);
    }

    #[test]
    fn accounting_categories_list_update_should_fail_when_category_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category = AccountingCategory::with_id(
            AccountingCategoryId::new(),
            "Non-existent Category".to_string(),
        );
        let category_id = category.id().clone();

        let result = categories_list.update(category);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountingCategoriesListError::NotFound(category_id)
        );
    }

    #[test]
    fn accounting_categories_list_delete_should_remove_existing_category() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Category to Delete";
        let category = categories_list.create(category_name).unwrap();

        categories_list.delete(category.id().clone()).unwrap();

        assert!(categories_list.categories().is_empty());
    }

    #[test]
    fn accounting_categories_list_delete_should_fail_when_category_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut categories_list = AccountingCategoriesList::new(repository);

        let non_existent_id = AccountingCategoryId::new();

        let result = categories_list.delete(non_existent_id.clone());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountingCategoriesListError::NotFound(non_existent_id)
        );
    }
}
