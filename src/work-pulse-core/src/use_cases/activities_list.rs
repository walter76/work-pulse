use std::{io::Read, sync::Arc, time::Instant};

use chrono::{NaiveDate, NaiveTime};
use thiserror::Error;
use tokio::sync::Mutex;

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
pub struct ActivitiesList<R> {
    /// The repository holding the list of activities.
    repository: Arc<Mutex<R>>,
}

impl<R: ActivitiesListRepository> ActivitiesList<R> {
    /// Creates a new `ActivitiesList`.
    ///
    /// # Arguments
    ///
    /// - `repository`: The repository holding the list of activities.
    pub fn new(repository: Arc<Mutex<R>>) -> Self {
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
    pub async fn record(
        &mut self,
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: Option<NaiveTime>,
        accounting_category_id: AccountingCategoryId,
        task: String,
    ) -> Activity {
        let mut repo = self.repository.lock().await;

        let mut activity = Activity::new(date, start_time, accounting_category_id, task);
        activity.set_end_time(end_time);

        repo.add(activity.clone()).await;

        activity
    }

    /// Returns the list of activities.
    pub async fn activities(&self) -> Vec<Activity> {
        let repo = self.repository.lock().await;
        repo.get_all().await
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
    pub async fn get_by_id(&self, activity_id: &ActivityId) -> Option<Activity> {
        let repo = self.repository.lock().await;

        repo.get_all()
            .await
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
    pub async fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListError> {
        let mut repo = self.repository.lock().await;

        let activity_id = activity.id().clone();
        repo.update(activity)
            .await
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
    pub async fn delete(&mut self, activity_id: ActivityId) -> Result<(), ActivitiesListError> {
        let mut repo = self.repository.lock().await;

        repo.delete(activity_id.clone())
            .await
            .map_err(|_| ActivitiesListError::NotFound(activity_id))
    }

    /// Imports activities from an external source using the provided importer.
    ///
    /// # Arguments
    ///
    /// - `importer`: The `ActivitiesImporter` implementation to use for importing activities.
    /// - `reader`: The reader from which to import activities.
    /// - `year`: The year to associate with the imported activities.
    /// - `delete_all`: Whether to delete all existing activities before importing new ones.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the import was successful.
    /// - `Err(ActivitiesImporterError)`: If an error occurred during the import process.
    pub async fn import<I: ActivitiesImporter, D: Read + Send>(
        &mut self,
        importer: &mut I,
        reader: D,
        year: u16,
        delete_all: bool,
    ) -> Result<(), ActivitiesImporterError> {
        let mut repo = self.repository.lock().await;

        let import_start = Instant::now();
        let activities = importer.import(reader, year).await?;
        let import_duration = import_start.elapsed();

        tracing::info!(
            duration_ms = import_duration.as_millis(),
            count = activities.len(),
            "Activities imported from source"
        );

        let db_start = Instant::now();

        if delete_all {
            repo.delete_all().await.map_err(|e| ActivitiesImporterError::RepositoryError(e.to_string()))?;
        }

        for activity in activities {
            repo.add(activity).await;
        }
        let db_duration = db_start.elapsed();

        tracing::info!(
            duration_ms = db_duration.as_millis(),
            "Activities saved to database"
        );

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
    use async_trait::async_trait;
    use chrono::{NaiveDate, NaiveTime};

    #[tokio::test]
    async fn record_should_add_activity_with_end_time() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Test Task".to_string(),
            )
            .await;

        let activities = activities_list.activities().await;
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0], activity);
    }

    #[tokio::test]
    async fn record_should_add_activity_without_end_time() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                None,
                AccountingCategoryId::new(),
                "Test Task".to_string(),
            )
            .await;

        let activities = activities_list.activities().await;
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0], activity);
    }

    #[tokio::test]
    async fn activities_should_return_empty_when_no_activities() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let activities_list = ActivitiesList::new(repository);

        assert!(activities_list.activities().await.is_empty());
    }

    #[tokio::test]
    async fn activities_should_return_all_activities() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Task 1".to_string(),
            )
            .await;

        activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 2).expect("Valid activity date"),
                NaiveTime::from_hms_opt(11, 0, 0).expect("Valid activity start time"),
                None,
                AccountingCategoryId::new(),
                "Task 2".to_string(),
            )
            .await;

        let activities = activities_list.activities().await;
        assert_eq!(activities.len(), 2);
    }

    #[tokio::test]
    async fn activities_list_get_by_id_should_return_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Test Task".to_string(),
            )
            .await;

        let retrieved_activity = activities_list.get_by_id(activity.id()).await.unwrap();
        assert_eq!(retrieved_activity, activity);
    }

    #[tokio::test]
    async fn activities_list_get_by_id_should_return_none_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let activities_list = ActivitiesList::new(repository);

        let non_existent_id = ActivityId::new();
        let result = activities_list.get_by_id(&non_existent_id).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn activities_list_update_should_modify_existing_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let mut activity = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Test Task".to_string(),
            )
            .await;

        activity.set_task("Updated Task".to_string());
        activities_list.update(activity.clone()).await.unwrap();

        let updated_activity = activities_list.get_by_id(activity.id()).await.unwrap();
        assert_eq!(updated_activity.task(), "Updated Task");
    }

    #[tokio::test]
    async fn activities_list_update_should_fail_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = Activity::with_id(
            ActivityId::new(),
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
            AccountingCategoryId::new(),
            "Non-existent Task".to_string(),
        );

        let result = activities_list.update(activity.clone()).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ActivitiesListError::NotFound(activity.id().clone())
        );
    }

    #[tokio::test]
    async fn activities_list_delete_should_remove_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date"),
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Test Task".to_string(),
            )
            .await;

        activities_list.delete(activity.id().clone()).await.unwrap();

        assert!(activities_list.activities().await.is_empty());
    }

    #[tokio::test]
    async fn activities_list_delete_should_fail_when_activity_not_found() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let non_existent_id = ActivityId::new();
        let result = activities_list.delete(non_existent_id.clone()).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ActivitiesListError::NotFound(non_existent_id)
        );
    }

    #[tokio::test]
    async fn activities_list_import_should_add_activities() {
        struct MockImporter;

        #[async_trait]
        impl ActivitiesImporter for MockImporter {
            async fn import<R: Read + Send>(
                &mut self,
                _reader: R,
                year: u16,
            ) -> Result<Vec<Activity>, ActivitiesImporterError> {
                let activity1 = Activity::with_id(
                    ActivityId::new(),
                    NaiveDate::from_ymd_opt(year as i32, 10, 1).expect("Valid activity date"),
                    NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                    AccountingCategoryId::new(),
                    "Imported Task 1".to_string(),
                );

                let activity2 = Activity::with_id(
                    ActivityId::new(),
                    NaiveDate::from_ymd_opt(year as i32, 10, 2).expect("Valid activity date"),
                    NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity start time"),
                    AccountingCategoryId::new(),
                    "Imported Task 2".to_string(),
                );

                Ok(vec![activity1, activity2])
            }
        }

        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let mut importer = MockImporter;
        let data = b"mock data";
        activities_list
            .import(&mut importer, &data[..], 2023, true)
            .await
            .unwrap();

        let activities = activities_list.activities().await;
        assert_eq!(activities.len(), 2);
        assert_eq!(activities[0].task(), "Imported Task 1");
        assert_eq!(activities[1].task(), "Imported Task 2");
    }

    #[tokio::test]
    async fn activities_list_import_should_delete_activities_before_import() {
        struct MockImporter;

        #[async_trait]
        impl ActivitiesImporter for MockImporter {
            async fn import<R: Read + Send>(
                &mut self,
                _reader: R,
                year: u16,
            ) -> Result<Vec<Activity>, ActivitiesImporterError> {
                let activity = Activity::with_id(
                    ActivityId::new(),
                    NaiveDate::from_ymd_opt(year as i32, 10, 1).expect("Valid activity date"),
                    NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                    AccountingCategoryId::new(),
                    "Imported Task".to_string(),
                );

                Ok(vec![activity])
            }
        }

        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        // Record an initial activity
        activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 9, 30).expect("Valid activity date"),
                NaiveTime::from_hms_opt(8, 0, 0).expect("Valid activity start time"),
                None,
                AccountingCategoryId::new(),
                "Initial Task".to_string(),
            )
            .await;

        let mut importer = MockImporter;
        let data = b"mock data";
        activities_list
            .import(&mut importer, &data[..], 2023, true)
            .await
            .unwrap();

        let activities = activities_list.activities().await;
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].task(), "Imported Task");
    }
}
