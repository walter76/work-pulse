use crate::entities::pam::PamCategory;

/// Represents a list of all PAM categories.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PamCategoriesList {
    /// The list of PAM categories.
    categories: Vec<PamCategory>,
}

impl PamCategoriesList {
    /// Creates a new `PamCategoriesList`.
    pub fn new() -> Self {
        Self {
            categories: vec![],
        }
    }

    /// Adds a PAM category to the list.
    /// 
    /// # Arguments
    /// 
    /// - `category_name`: The name of the PAM category to add.
    pub fn create(&mut self, category_name: &str) {
        // TODO Avoid creating categories with the same name.

        let category = PamCategory::new(category_name.to_string());
        self.categories.push(category);
    }

    /// Returns the list of PAM categories.
    pub fn categories(&self) -> &[PamCategory] {
        &self.categories
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pam_categories_list_create_should_add_category() {
        let mut categories_list = PamCategoriesList::new();
        let category_name = "Test Category";
        categories_list.create(category_name);

        assert_eq!(categories_list.categories().len(), 1);
        assert_eq!(categories_list.categories()[0].name, category_name);
    }

    #[test]
    fn pam_categories_list_should_return_empty_when_no_categories() {
        let categories_list = PamCategoriesList::new();
        assert!(categories_list.categories().is_empty());
    }

    #[test]
    fn pam_categories_list_should_return_all_categories() {
        let mut categories_list = PamCategoriesList::new();
        categories_list.create("Category 1");
        categories_list.create("Category 2");

        let categories = categories_list.categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name, "Category 1");
        assert_eq!(categories[1].name, "Category 2");
    }    
}