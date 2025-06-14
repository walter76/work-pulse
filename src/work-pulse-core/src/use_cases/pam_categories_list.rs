use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::{adapters::PamCategoriesListRepository, entities::pam::{PamCategory, PamCategoryId}};

/// Represents an error that can occur while managing PAM categories.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum PamCategoriesListError {
    /// A PAM category with the specified name already exists.
    #[error("A PAM category with the name '{0}' already exists.")]
    PamCategoryAlreadyExists(String),

    /// A PAM category with the ID does not exists.
    #[error("PAM category with the ID `{0}` does not exists.")]
    NotFound(PamCategoryId),    
}

/// Represents a list of all PAM categories.
pub struct PamCategoriesList {
    /// The list of PAM categories.
    repository: Arc<Mutex<dyn PamCategoriesListRepository>>,
}

impl PamCategoriesList {
    /// Creates a new `PamCategoriesList`.
    /// 
    /// # Arguments
    /// 
    /// - `repository`: An `Arc<Mutex<dyn PamCategoriesListRepository>>` that provides access to the PAM categories repository.
    pub fn new(repository: Arc<Mutex<dyn PamCategoriesListRepository>>) -> Self {
        Self { repository }
    }

    /// Creates a new `PamCategoriesList` with test data.
    /// 
    /// # Arguments
    /// 
    /// - `repository`: An `Arc<Mutex<dyn PamCategoriesListRepository>>` that provides access to the PAM categories repository.
    pub fn with_test_data(repository: Arc<Mutex<dyn PamCategoriesListRepository>>) -> Self {
        // FIXME Remove this test data creation
        let mut pam_categories_list = Self { repository };

        pam_categories_list.create("Current Version").unwrap();
        pam_categories_list.create("SWA Trainer").unwrap();
        pam_categories_list.create("Techno Cluster").unwrap();
        pam_categories_list.create("Other").unwrap();

        pam_categories_list
    }

    /// Adds a PAM category to the list.
    /// 
    /// # Arguments
    /// 
    /// - `category_name`: The name of the PAM category to add.
    /// 
    /// # Returns
    /// 
    /// - `Ok(PamCategory)`: If the category was successfully created.
    /// - `Err(PamCategoriesListError)`: If a category with the same name already exists.
    pub fn create(&mut self, category_name: &str) -> Result<PamCategory, PamCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        // Check if a category with the same name already exists.
        if repo.get_all().iter()
            .find(|category| category.name() == category_name)
            .is_some() {
            return Err(PamCategoriesListError::PamCategoryAlreadyExists(category_name.to_string()));
        }

        let pam_category = PamCategory::new(category_name.to_string());
        repo.add(pam_category.clone());

        Ok(pam_category)
    }

    /// Returns the list of PAM categories.
    /// 
    /// # Returns
    /// 
    /// - `Vec<PamCategory>`: A vector containing all PAM categories.
    pub fn categories(&self) -> Vec<PamCategory> {
        let repo = self.repository.lock().unwrap();
        repo.get_all()
    }

    /// Updates an existing PAM category in the list.
    /// 
    /// # Arguments
    /// 
    /// - `category`: The `PamCategory` instance with updated information to be saved in the repository.
    /// 
    /// # Returns
    /// 
    /// - `Ok(())`: If the category was successfully updated.
    /// - `Err(PamCategoriesListError)`: If the category with the specified ID does not exist.
    pub fn update(&mut self, category: PamCategory) -> Result<(), PamCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        let category_id = category.id().clone();
        repo.update(category).map_err(|_| PamCategoriesListError::NotFound(category_id))?;

        Ok(())
    }

    /// Deletes a PAM category from the list.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier of the PAM category to be deleted.
    /// 
    /// # Returns
    /// 
    /// - `Ok(())`: If the category was successfully deleted.
    /// - `Err(PamCategoriesListError)`: If the category with the specified ID does not exist.
    pub fn delete(&mut self, id: PamCategoryId) -> Result<(), PamCategoriesListError> {
        let mut repo = self.repository.lock().unwrap();

        repo.delete(id.clone()).map_err(|_| PamCategoriesListError::NotFound(id))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repositories::in_memory::pam_categories_list::InMemoryPamCategoriesListRepository;

    use super::*;

    #[test]
    fn pam_categories_list_create_should_add_category() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).unwrap();

        assert_eq!(categories_list.categories().len(), 1);
        assert_eq!(categories_list.categories()[0].name(), category_name);
    }

    #[test]
    fn pam_categories_list_create_should_fail_when_category_exists() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name).unwrap();

        let result = categories_list.create(category_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamCategoriesListError::PamCategoryAlreadyExists(category_name.to_string()));
    }
    
    #[test]
    fn pam_categories_list_should_return_empty_when_no_categories() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let categories_list = PamCategoriesList::new(repository);

        assert!(categories_list.categories().is_empty());
    }

    #[test]
    fn pam_categories_list_should_return_all_categories() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);
        
        categories_list.create("Category 1").unwrap();
        categories_list.create("Category 2").unwrap();

        let categories = categories_list.categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name(), "Category 1");
        assert_eq!(categories[1].name(), "Category 2");
    }

    #[test]
    fn pam_categories_list_update_should_modify_existing_category() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

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
    fn pam_categories_list_update_should_fail_when_category_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let category = PamCategory::with_id(PamCategoryId::new(), "Non-existent Category".to_string());
        let category_id = category.id().clone();
        
        let result = categories_list.update(category);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamCategoriesListError::NotFound(category_id));
    }

    #[test]
    fn pam_categories_list_delete_should_remove_existing_category() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let category_name = "Category to Delete";
        let category = categories_list.create(category_name).unwrap();
        
        categories_list.delete(category.id().clone()).unwrap();

        assert!(categories_list.categories().is_empty());
    }

    #[test]
    fn pam_categories_list_delete_should_fail_when_category_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let non_existent_id = PamCategoryId::new();
        
        let result = categories_list.delete(non_existent_id.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamCategoriesListError::NotFound(non_existent_id));
    }
}