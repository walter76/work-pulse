use thiserror::Error;

use crate::entities::{activity::{Activity, ActivityId}, pam::{PamCategory, PamCategoryId}};

/// Error type for the PAM categories list repository.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum PamCategoriesListRepositoryError {
    /// Error indicating that the requested PAM category was not found.
    #[error("PAM category with ID {0} not found")]
    NotFound(PamCategoryId),
}

/// Repository trait for managing PAM categories.
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

    /// Deletes a PAM category from the repository.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier of the PAM category to be deleted.
    /// 
    /// # Returns
    /// 
    /// `Result<(), PamCategoriesListRepositoryError>` indicating success or failure of the delete operation.
    fn delete(&mut self, id: PamCategoryId) -> Result<(), PamCategoriesListRepositoryError>;

    /// Retrieves a PAM category by its name, or creates it if it does not exist.
    /// 
    /// # Arguments
    /// 
    /// - `name`: The name of the PAM category to retrieve or create.
    /// 
    /// # Returns
    /// 
    /// `Result<PamCategory, PamCategoriesListRepositoryError>` containing the retrieved or newly created PAM category.
    fn get_or_create_by_name(&mut self, name: &str) -> Result<PamCategory, PamCategoriesListRepositoryError>;
}

/// Error type for the activities list repository.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ActivitiesListRepositoryError {
    /// An error indicating that the activity with the specified ID was not found.
    #[error("Activity with ID {0} not found")]
    NotFound(ActivityId),
}

/// Repository trait for managing a list of activities.
pub trait ActivitiesListRepository: Send + Sync {
    /// Retrieves a list of all activities.
    /// 
    /// # Returns
    /// 
    /// A vector of `Activity` instances representing all activities in the repository.
    fn get_all(&self) -> Vec<Activity>;

    /// Adds a new activity to the list.
    /// 
    /// # Arguments
    /// 
    /// - `activity`: The `Activity` instance to be added to the list.
    fn add(&mut self, activity: Activity);

    /// Updates an existing activity in the repository.
    /// 
    /// # Arguments
    /// 
    /// - `activity`: The `Activity` instance with updated information to be saved in the repository.
    /// 
    /// # Returns
    /// 
    /// `Result<(), ActivitiesListRepositoryError>` indicating success or failure of the update operation.
    fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListRepositoryError>;

    /// Deletes an activity from the repository.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier of the activity to be deleted.
    /// 
    /// # Returns
    /// 
    /// `Result<(), ActivitiesListRepositoryError>` indicating success or failure of the delete operation.
    fn delete(&mut self, id: ActivityId) -> Result<(), ActivitiesListRepositoryError>;    
}

/// Error type for the activities importer.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ActivitiesImporterError {
    #[error("Could not parse the activities from the source")]
    ParseError,
}

pub trait ActivitiesImporter {
    /// Imports activities from a source.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the import operation.
    fn import(&mut self) -> Result<Vec<Activity>, ActivitiesImporterError>;
}
