use thiserror::Error;

use crate::entities::pam::{PamCategory, PamCategoryId};

/// Error type for the PAM categories list repository.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum PamCategoriesListRepositoryError {
    /// Error indicating that the requested PAM category was not found.
    #[error("PAM category with ID {0} not found")]
    NotFound(PamCategoryId),
}

pub trait PamCategoriesListRepository: Send + Sync {
    /// Retrieves a list of all PAM categories.
    ///
    /// # Returns
    ///
    /// A vector of `PamCategory` instances representing all available PAM categories.
    fn get_all(&self) -> Vec<PamCategory>;

    /// Retrieves a specific PAM category by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier of the PAM category to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<PamCategory>` that contains the requested category if found, or `None` if not found.
    fn get_by_id(&self, id: PamCategoryId) -> Option<PamCategory>;

    /// Adds a new PAM category to the repository.
    /// 
    /// # Arguments
    /// 
    /// - `category`: The `PamCategory` instance to be added to the repository.
    fn add(&mut self, category: PamCategory);

    /// Updates an existing PAM category in the repository.
    /// 
    /// # Arguments
    /// 
    /// - `category`: The `PamCategory` instance with updated information to be saved in the repository.
    /// 
    /// # Returns
    /// 
    /// `Result<(), PamCategoriesListRepositoryError>` indicating success or failure of the update operation.
    fn update(&mut self, category: PamCategory) -> Result<(), PamCategoriesListRepositoryError>;
}
