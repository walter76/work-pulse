use std::{
    io::Read,
    sync::{Arc, Mutex},
};

use chrono::{NaiveDate, NaiveTime};
use thiserror::Error;

use crate::{
    adapters::{ActivitiesImporter, ActivitiesImporterError, ActivitiesListRepository},
    entities::{
        accounting::AccountingCategoryId,
        activity::{Activity, ActivityId},
    },
};

/// Represents an error that can occur while managing the list of activities.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ActivitiesListError {
    /// An activity with the ID does not exists.
    #[error("Activity with the ID `{0}` does not exists.")]
    NotFound(ActivityId),
}

/// Represents a list of activities.
///
/// It is used to record activities that the user did during his working day.
pub struct ActivitiesList {
    /// The repository for a list of activities.
    repository: Arc<Mutex<dyn ActivitiesListRepository>>,
}

impl ActivitiesList {
    /// Creates a new `ActivitiesList`.
    ///
    /// # Arguments
    ///
    /// - `repository`: The repository for a list of activities.
    pub fn new(repository: Arc<Mutex<dyn ActivitiesListRepository>>) -> Self {
        Self { repository }
    }

    /// Adds an activity to the list.
    ///
    /// # Arguments
    ///
    /// - `date`: The date when the activity was performed.
    /// - `start_time`: The time when the activity started.
    /// - `end_time`: The time when the activity ended, if applicable.
    /// - `accounting_category_id`: The accounting category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    ///
    /// # Returns
    ///
    /// - `Activity`: The created activity.
    pub fn record(
        &mut self,
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: Option<NaiveTime>,
        accounting_category_id: AccountingCategoryId,
        task: String,
    ) -> Activity {
        let mut repo = self.repository.lock().expect("Failed to lock repository");

        let mut activity = Activity::new(date, start_time, accounting_category_id, task);
        activity.set_end_time(end_time);

        repo.add(activity.clone());

        activity
    }

    /// Returns the list of activities.
    pub fn activities(&self) -> Vec<Activity> {
        let repo = self.repository.lock().expect("Failed to lock repository");
        repo.get_all()
    }

    /// Retrieves an activity by its ID.
    ///
    /// # Arguments
    ///
    /// - `activity_id`: The ID of the activity to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(Activity)`: If the activity was found.
    /// - `None`: If the activity with the specified ID does not exist.
    pub fn get_by_id(&self, activity_id: &ActivityId) -> Option<Activity> {
        let repo = self.repository.lock().expect("Failed to lock repository");

        repo.get_all()
            .iter()
            .find(|activity| activity.id() == activity_id)
            .cloned()
    }

    /// Updates an existing activity in the list.
    ///
    /// # Arguments
    ///
    /// - `activity`: The `Activity` instance with updated information to be saved.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the activity was successfully updated.
    /// - `Err(ActivitiesListError)`: If the activity with the specified ID does not exist.
    pub fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListError> {
        let mut repo = self.repository.lock().unwrap();

        let activity_id = activity.id().clone();
        repo.update(activity)
            .map_err(|_| ActivitiesListError::NotFound(activity_id))
    }

    /// Deletes an activity from the list.
    ///
    /// # Arguments
    ///
    /// - `activity_id`: The ID of the activity to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the activity was successfully deleted.
    /// - `Err(ActivitiesListError)`: If the activity with the specified ID does
    pub fn delete(&mut self, activity_id: ActivityId) -> Result<(), ActivitiesListError> {
        let mut repo = self.repository.lock().unwrap();

        repo.delete(activity_id.clone())
            .map_err(|_| ActivitiesListError::NotFound(activity_id))
    }

    /// Imports activities from an external source using the provided importer.
    ///
    /// # Arguments
    ///
    /// - `importer`: The `ActivitiesImporter` implementation to use for importing activities.
    /// - `reader`: The reader from which to import activities.
    /// - `year`: The year to associate with the imported activities.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the import was successful.
    /// - `Err(ActivitiesImporterError)`: If an error occurred during the import process.
    pub fn import<I: ActivitiesImporter, R: Read>(
        &mut self,
        importer: &mut I,
        reader: R,
        year: u16,
    ) -> Result<(), ActivitiesImporterError> {
        let mut repo = self.repository.lock().unwrap();

        let activities = importer.import(reader, year)?;

        for activity in activities {
            repo.add(activity);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::accounting::AccountingCategoryId,
        infra::repositories::in_memory::activities_list::InMemoryActivitiesListRepository,
    };
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn record_should_add_activity_with_end_time() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        assert_eq!(activities_list.activities().len(), 1);
        assert_eq!(activities_list.activities()[0], activity);
    }

    #[test]
    fn record_should_add_activity_without_end_time() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            None,
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        assert_eq!(activities_list.activities().len(), 1);
        assert_eq!(activities_list.activities()[0], activity);
    }

    #[test]
    fn activities_should_return_empty_when_no_activities() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let activities_list = ActivitiesList::new(repository);

        assert!(activities_list.activities().is_empty());
    }

    #[test]
    fn activities_should_return_all_activities() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            AccountingCategoryId::new(),
            "Task 1".to_string(),
        );

        activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 2).expect("Valid date"),
            NaiveTime::from_hms_opt(11, 0, 0).expect("Valid time"),
            None,
            AccountingCategoryId::new(),
            "Task 2".to_string(),
        );

        let activities = activities_list.activities();
        assert_eq!(activities.len(), 2);
    }

    #[test]
    fn activities_list_get_by_id_should_return_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        let retrieved_activity = activities_list.get_by_id(activity.id()).unwrap();
        assert_eq!(retrieved_activity, activity);
    }

    #[test]
    fn activities_list_get_by_id_should_return_none_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let activities_list = ActivitiesList::new(repository);

        let non_existent_id = ActivityId::new();
        let result = activities_list.get_by_id(&non_existent_id);

        assert!(result.is_none());
    }

    #[test]
    fn activities_list_update_should_modify_existing_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let mut activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        activity.set_task("Updated Task".to_string());
        activities_list.update(activity.clone()).unwrap();

        let updated_activity = activities_list.get_by_id(activity.id()).unwrap();
        assert_eq!(updated_activity.task(), "Updated Task");
    }

    #[test]
    fn activities_list_update_should_fail_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = Activity::with_id(
            ActivityId::new(),
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            AccountingCategoryId::new(),
            "Non-existent Task".to_string(),
        );

        let result = activities_list.update(activity.clone());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ActivitiesListError::NotFound(activity.id().clone())
        );
    }

    #[test]
    fn activities_list_delete_should_remove_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        activities_list.delete(activity.id().clone()).unwrap();

        assert!(activities_list.activities().is_empty());
    }

    #[test]
    fn activities_list_delete_should_fail_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let non_existent_id = ActivityId::new();
        let result = activities_list.delete(non_existent_id.clone());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ActivitiesListError::NotFound(non_existent_id)
        );
    }
}
