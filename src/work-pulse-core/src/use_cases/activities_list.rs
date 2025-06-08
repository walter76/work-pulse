use std::sync::{Arc, Mutex};

use chrono::{NaiveDate, NaiveTime};

use crate::{adapters::ActivitiesListRepository, entities::{activity::Activity, pam::PamCategoryId}};

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
        Self {
            repository,
        }
    }

    /// Adds an activity to the list.
    /// 
    /// # Arguments
    /// 
    /// - `date`: The date when the activity was performed.
    /// - `start_time`: The time when the activity started.
    /// - `end_time`: The time when the activity ended, if applicable.
    /// - `pam_category_id`: The PAM category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    /// 
    /// # Returns
    /// 
    /// - `Activity`: The created activity.
    pub fn record(&mut self, date: NaiveDate, start_time: NaiveTime, end_time: Option<NaiveTime>, pam_category_id: PamCategoryId, task: String) -> Activity {
        let mut repo = self.repository.lock().expect("Failed to lock repository");

        let mut activity = Activity::new(date, start_time, pam_category_id, task);
        activity.set_end_time(end_time);

        repo.add(activity.clone());

        activity
    }

    /// Returns the list of activities.
    pub fn activities(&self) -> Vec<Activity> {
        let repo = self.repository.lock().expect("Failed to lock repository");
        repo.get_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::{pam::PamCategoryId}, infra::repositories::in_memory::activities_list::InMemoryActivitiesListRepository};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn record_should_add_activity_with_end_time() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid time")),
            PamCategoryId::new(),
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
            PamCategoryId::new(),
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
            PamCategoryId::new(),
            "Task 1".to_string(),
        );

        activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 2).expect("Valid date"),
            NaiveTime::from_hms_opt(11, 0, 0).expect("Valid time"),
            None,
            PamCategoryId::new(),
            "Task 2".to_string(),
        );

        let activities = activities_list.activities();
        assert_eq!(activities.len(), 2);
    }
}