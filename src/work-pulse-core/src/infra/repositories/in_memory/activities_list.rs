use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{
    adapters::{ActivitiesListRepository, ActivitiesListRepositoryError},
    entities::{
        accounting::AccountingCategoryId,
        activity::{Activity, ActivityId},
    },
};

/// Represents a record for an `Activity`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ActivityRecord {
    /// The unique identifier for the record.
    id: Uuid,

    /// The date when the activity was performed.
    date: NaiveDate,

    /// The time when the activity started.
    start_time: NaiveTime,

    /// The time when the activity ended, if applicable.
    end_time: Option<NaiveTime>,

    /// The accounting category ID associated with the activity.
    accounting_category_id: AccountingCategoryId,

    /// The task itself.
    task: String,
}

impl ActivityRecord {
    /// Converts an `Activity` entity to an `ActivityRecord`.
    ///
    /// # Arguments
    ///
    /// - `activity`: The `Activity` entity to convert.
    fn from_entity(activity: Activity) -> Self {
        ActivityRecord {
            id: activity.id().0,
            date: activity.date().clone(),
            start_time: activity.start_time().clone(),
            end_time: activity.end_time().cloned(),
            accounting_category_id: activity.accounting_category_id().clone(),
            task: activity.task().to_string(),
        }
    }

    /// Converts an `ActivityRecord` to an `Activity` entity.
    fn to_entity(&self) -> Activity {
        let mut activity = Activity::with_id(
            ActivityId(self.id),
            self.date,
            self.start_time,
            self.accounting_category_id.clone(),
            self.task.clone(),
        );
        activity.set_end_time(self.end_time);

        activity
    }
}

/// In-memory implementation of a repository for activities list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InMemoryActivitiesListRepository {
    activities: Vec<ActivityRecord>,
}

impl InMemoryActivitiesListRepository {
    /// Creates a new in-memory repository for activities.
    pub fn new() -> Self {
        InMemoryActivitiesListRepository {
            activities: Vec::new(),
        }
    }
}

impl ActivitiesListRepository for InMemoryActivitiesListRepository {
    fn get_all(&self) -> Vec<Activity> {
        self.activities
            .iter()
            .map(|record| record.to_entity())
            .collect()
    }

    fn get_by_date(&self, date: NaiveDate) -> Vec<Activity> {
        self.activities
            .iter()
            .filter(|record| record.date == date)
            .map(|record| record.to_entity())
            .collect()
    }
    
    fn get_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<Activity> {
        self.activities
            .iter()
            .filter(|record| record.date >= start && record.date <= end)
            .map(|record| record.to_entity())
            .collect()
    }
    
    fn add(&mut self, activity: Activity) {
        let record = ActivityRecord::from_entity(activity);
        self.activities.push(record);
    }

    fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListRepositoryError> {
        if let Some(record) = self.activities.iter_mut().find(|r| r.id == activity.id().0) {
            *record = ActivityRecord::from_entity(activity);

            Ok(())
        } else {
            Err(ActivitiesListRepositoryError::NotFound(
                activity.id().clone(),
            ))
        }
    }

    fn delete(&mut self, id: ActivityId) -> Result<(), ActivitiesListRepositoryError> {
        if let Some(index) = self.activities.iter().position(|record| record.id == id.0) {
            self.activities.remove(index);
            Ok(())
        } else {
            Err(ActivitiesListRepositoryError::NotFound(id))
        }
    }
}
