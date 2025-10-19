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
pub struct AccountingCategoriesList<R> {
    /// The repository that provides access to the accounting categories.
    repository: R,
}

impl<R: AccountingCategoriesListRepository> AccountingCategoriesList<R> {
    /// Creates a new `AccountingCategoriesList`.
    ///
    /// # Arguments
    ///
    /// - `repository`: An `Arc<Mutex<dyn AccountingCategoriesListRepository>>` that provides access to the accounting categories repository.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Createss a new accounting category and adds it to the list.
    ///
    /// # Arguments
    ///
    /// - `category_name`: The name of the accounting category to create.
    ///
    /// # Returns
    ///
    /// - `Ok(AccountingCategory)`: If the category was successfully created.
    /// - `Err(AccountingCategoriesListError)`: If a category with the same name already exists.
    pub async fn create(
        &mut self,
        category_name: &str,
    ) -> Result<AccountingCategory, AccountingCategoriesListError> {
        // Check if a category with the same name already exists.
        if self
            .repository
            .get_all()
            .await
            .iter()
            .find(|category| category.name() == category_name)
            .is_some()
        {
            return Err(
                AccountingCategoriesListError::AccountingCategoryAlreadyExists(
                    category_name.to_string(),
                ),
            );
        }

        let accounting_category = AccountingCategory::new(category_name.to_string());
        self.repository.add(accounting_category.clone());

        Ok(accounting_category)
    }

    /// Returns the list of accounting categories.
    ///
    /// # Returns
    ///
    /// - `Vec<AccountingCategory>`: A vector containing all accounting categories.
    pub async fn categories(&self) -> Vec<AccountingCategory> {
        self.repository.get_all().await
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
    pub fn update(
        &mut self,
        category: AccountingCategory,
    ) -> Result<(), AccountingCategoriesListError> {
        let category_id = category.id().clone();

        self.repository
            .update(category)
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
    pub fn delete(
        &mut self,
        id: AccountingCategoryId,
    ) -> Result<(), AccountingCategoriesListError> {
        self.repository
            .delete(id.clone())
            .map_err(|_| AccountingCategoriesListError::NotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repositories::in_memory::accounting_categories_list::InMemoryAccountingCategoriesListRepository;

    use super::*;

    #[tokio::test]
    async fn accounting_categories_list_create_should_add_category() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).await.unwrap();

        assert_eq!(categories_list.categories().await.len(), 1);
        assert_eq!(categories_list.categories().await[0].name(), category_name);
    }

    #[tokio::test]
    async fn accounting_categories_list_create_should_fail_when_category_exists() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).await.unwrap();

        let result = categories_list.create(category_name).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountingCategoriesListError::AccountingCategoryAlreadyExists(
                category_name.to_string()
            )
        );
    }

    #[tokio::test]
    async fn accounting_categories_list_should_return_empty_when_no_categories() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let categories_list = AccountingCategoriesList::new(repository);

        assert!(categories_list.categories().await.is_empty());
    }

    #[tokio::test]
    async fn accounting_categories_list_should_return_all_categories() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let mut categories_list = AccountingCategoriesList::new(repository);

        categories_list.create("Category 1").await.unwrap();
        categories_list.create("Category 2").await.unwrap();

        let categories = categories_list.categories().await;
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name(), "Category 1");
        assert_eq!(categories[1].name(), "Category 2");
    }

    #[tokio::test]
    async fn accounting_categories_list_update_should_modify_existing_category() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Original Category";
        let mut category = categories_list.create(category_name).await.unwrap();

        let updated_name = "Updated Category";
        category.set_name(updated_name.to_string());

        categories_list.update(category).unwrap();

        let categories = categories_list.categories().await;
        let actual_name = categories.first().map(|c| c.name()).unwrap();
        assert_eq!(actual_name, updated_name);
    }

    #[test]
    fn accounting_categories_list_update_should_fail_when_category_not_found() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
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

    #[tokio::test]
    async fn accounting_categories_list_delete_should_remove_existing_category() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
        let mut categories_list = AccountingCategoriesList::new(repository);

        let category_name = "Category to Delete";
        let category = categories_list.create(category_name).await.unwrap();

        categories_list.delete(category.id().clone()).unwrap();

        assert!(categories_list.categories().await.is_empty());
    }

    #[test]
    fn accounting_categories_list_delete_should_fail_when_category_not_found() {
        let repository = InMemoryAccountingCategoriesListRepository::new();
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
