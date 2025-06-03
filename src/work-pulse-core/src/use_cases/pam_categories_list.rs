use std::sync::{Arc, Mutex};

use crate::{adapters::PamCategoriesListRepository, entities::pam::PamCategory};

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

        pam_categories_list.create("Current Version");
        pam_categories_list.create("SWA Trainer");
        pam_categories_list.create("Techno Cluster");
        pam_categories_list.create("Other");

        pam_categories_list
    }

    /// Adds a PAM category to the list.
    /// 
    /// # Arguments
    /// 
    /// - `category_name`: The name of the PAM category to add.
    pub fn create(&mut self, category_name: &str) -> PamCategory {
        // TODO Avoid creating categories with the same name.

        let pam_category = PamCategory::new(category_name.to_string());

        let mut repo = self.repository.lock().unwrap();
        repo.add(pam_category.clone());

        pam_category
    }

    /// Returns the list of PAM categories.
    pub fn categories(&self) -> Vec<PamCategory> {
        let repo = self.repository.lock().unwrap();
        repo.get_all()
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repositories::InMemoryPamCategoriesListRepository;

    use super::*;

    #[test]
    fn pam_categories_list_create_should_add_category() {
        let repository = Arc::new(Mutex::new(InMemoryPamCategoriesListRepository::new()));
        let mut categories_list = PamCategoriesList::new(repository);

        let category_name = "Test Category";
        categories_list.create(category_name);

        assert_eq!(categories_list.categories().len(), 1);
        assert_eq!(categories_list.categories()[0].name(), category_name);
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
        
        categories_list.create("Category 1");
        categories_list.create("Category 2");

        let categories = categories_list.categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name(), "Category 1");
        assert_eq!(categories[1].name(), "Category 2");
    }    
}