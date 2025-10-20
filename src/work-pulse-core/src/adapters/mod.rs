use std::io::Read;

use async_trait::async_trait;
use chrono::NaiveDate;
use thiserror::Error;

use crate::entities::{
    accounting::{AccountingCategory, AccountingCategoryId},
    activity::{Activity, ActivityId},
};

/// Error type for the accounting categories list repository.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum AccountingCategoriesListRepositoryError {
    /// Error indicating that the requested accounting category was not found.
    #[error("Accounting category with ID {0} not found")]
    NotFound(AccountingCategoryId),

    /// Error indicating a database-related issue.
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Repository trait for managing accounting categories.
#[async_trait]
pub trait AccountingCategoriesListRepository: Send + Sync {
    /// Retrieves a list of all accounting categories.
    ///
    /// # Returns
    ///
    /// A vector of `AccountingCategory` instances representing all available accounting categories.
    async fn get_all(&self) -> Vec<AccountingCategory>;

    /// Retrieves a specific accounting category by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier of the accounting category to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<AccountingCategory>` that contains the requested category if found, or `None` if not found.
    async fn get_by_id(&self, id: AccountingCategoryId) -> Option<AccountingCategory>;

    /// Adds a new accounting category to the repository.
    ///
    /// # Arguments
    ///
    /// - `category`: The `AccountingCategory` instance to be added to the repository.
    async fn add(&mut self, category: AccountingCategory);

    /// Updates an existing accounting category in the repository.
    ///
    /// # Arguments
    ///
    /// - `category`: The `AccountingCategory` instance with updated information to be saved in the repository.
    ///
    /// # Returns
    ///
    /// `Result<(), AccountingCategoriesListRepositoryError>` indicating success or failure of the update operation.
    async fn update(
        &mut self,
        category: AccountingCategory,
    ) -> Result<(), AccountingCategoriesListRepositoryError>;

    /// Deletes an accounting category from the repository.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier of the accounting category to be deleted.
    ///
    /// # Returns
    ///
    /// `Result<(), AccountingCategoriesListRepositoryError>` indicating success or failure of the delete operation.
    async fn delete(
        &mut self,
        id: AccountingCategoryId,
    ) -> Result<(), AccountingCategoriesListRepositoryError>;

    /// Retrieves an accounting category by its name, or creates it if it does not exist.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the accounting category to retrieve or create.
    ///
    /// # Returns
    ///
    /// `Result<AccountingCategory, AccountingCategoriesListRepositoryError>` containing the retrieved or newly created accounting category.
    async fn get_or_create_by_name(
        &mut self,
        name: &str,
    ) -> Result<AccountingCategory, AccountingCategoriesListRepositoryError>;
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

    /// Retrieves a list of activities for a specific date.
    /// 
    /// # Arguments
    /// 
    /// - `date`: The date for which to retrieve activities.
    /// 
    /// # Returns
    /// A vector of `Activity` instances representing all activities for the specified date.
    fn get_by_date(&self, date: NaiveDate) -> Vec<Activity>;

    /// Retrieves a list of activities within a specified date range.
    /// 
    /// # Arguments
    /// 
    /// - `start`: The start date of the range (inclusive).
    /// - `end`: The end date of the range (inclusive).
    /// 
    /// # Returns
    /// A vector of `Activity` instances representing all activities within the specified date range.
    fn get_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<Activity>;

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

#[async_trait]
pub trait ActivitiesImporter {
    /// Imports activities from a source.
    ///
    /// # Arguments
    ///
    /// - `reader`: A reader instance that provides the source data for importing activities.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the import operation.
    async fn import<R: Read>(
        &mut self,
        reader: R,
        year: u16,
    ) -> Result<Vec<Activity>, ActivitiesImporterError>;
}
