use std::fmt::Display;

use chrono::{Duration, NaiveDate, NaiveTime};
use thiserror::Error;
use uuid::Uuid;

use super::accounting::AccountingCategoryId;

/// Errors that can occur when working with `ActivityId`.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ActivityIdError {
    /// The given string is not a valid activity id.
    #[error("The provided string is not a valid activity id: {0}")]
    NotAValidId(String),
}

/// The unique identifier for an activity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityId(pub Uuid);

impl ActivityId {
    /// Creates a new `ActivityId` with a random UUID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a string into an `ActivityId`.
    ///
    /// # Arguments
    ///
    /// - `s`: A string slice that represents a UUID.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ActivityId` if successful, or an `ActivityIdError` if the string is not a valid UUID.
    pub fn parse_str(s: &str) -> Result<Self, ActivityIdError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| ActivityIdError::NotAValidId(s.to_string()))
    }
}

impl Display for ActivityId {
    /// Formats the `ActivityId` as a string.
    ///
    /// # Arguments
    ///
    /// - `f`: A mutable reference to a formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents an activity that the user did during his working day.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Activity {
    /// The unique identifier for the activity.
    id: ActivityId,

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

impl Activity {
    /// Creates a new `Activity` with a random ID.
    ///
    /// # Arguments
    ///
    /// - `date`: The date when the activity was performed.
    /// - `start_time`: The time when the activity started.
    /// - `accounting_category_id`: The accounting category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    pub fn new(
        date: NaiveDate,
        start_time: NaiveTime,
        accounting_category_id: AccountingCategoryId,
        task: String,
    ) -> Self {
        Self {
            id: ActivityId::new(),
            date,
            start_time,
            end_time: None,
            accounting_category_id,
            task,
        }
    }

    /// Creates a new `Activity` with a specific ID.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier for the activity.
    /// - `date`: The date when the activity was performed.
    /// - `start_time`: The time when the activity started.
    /// - `accounting_category_id`: The accounting category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    pub fn with_id(
        id: ActivityId,
        date: NaiveDate,
        start_time: NaiveTime,
        accounting_category_id: AccountingCategoryId,
        task: String,
    ) -> Self {
        Self {
            id,
            date,
            start_time,
            end_time: None,
            accounting_category_id,
            task,
        }
    }

    /// Returns the unique identifier for the activity.
    pub fn id(&self) -> &ActivityId {
        &self.id
    }

    /// Returns the date when the activity was performed.
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// Returns the time when the activity started.
    pub fn start_time(&self) -> &NaiveTime {
        &self.start_time
    }

    /// Returns the time when the activity ended, if applicable.
    pub fn end_time(&self) -> Option<&NaiveTime> {
        self.end_time.as_ref()
    }

    /// Sets the end time for the activity.
    ///
    /// # Arguments
    ///
    /// - `end_time`: The time when the activity ended, if applicable.
    pub fn set_end_time(&mut self, end_time: Option<NaiveTime>) {
        self.end_time = end_time;
    }

    /// Returns the accounting category ID associated with the activity.
    pub fn accounting_category_id(&self) -> &AccountingCategoryId {
        &self.accounting_category_id
    }

    /// Sets the accounting category ID associated with the activity.
    ///
    /// # Arguments
    ///
    /// - `accounting_category_id`: The accounting category ID to associate with the activity.
    pub fn set_accounting_category_id(&mut self, accounting_category_id: AccountingCategoryId) {
        self.accounting_category_id = accounting_category_id;
    }

    /// Returns the task associated with the activity, if any.
    pub fn task(&self) -> &str {
        self.task.as_str()
    }

    /// Sets the task associated with the activity.
    ///
    /// # Arguments
    ///
    /// - `task`: An optional string representing the task associated with the activity.
    pub fn set_task(&mut self, task: String) {
        self.task = task;
    }

    /// Calculates the duration of the activity.
    /// If the end time is not set, the duration is considered to be zero.
    ///
    /// # Returns
    ///
    /// The duration of the activity as a `Duration`.
    pub fn duration(&self) -> Duration {
        if let Some(end_time) = self.end_time() {
            *end_time - *self.start_time()
        } else {
            Duration::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_id_new_should_create_activity_with_id() {
        let id = ActivityId::new();
        assert!(id.0.is_nil() == false);
    }

    #[test]
    fn activity_id_parse_str_should_parse_valid_id() {
        let unique_id = Uuid::new_v4();
        let activity_id = ActivityId::parse_str(unique_id.to_string().as_str()).unwrap();

        assert_eq!(unique_id, activity_id.0);
    }

    #[test]
    fn activity_id_parse_str_should_fail_with_invalid_id() {
        let invalid_id = "invalid-uuid";
        let result = ActivityId::parse_str(invalid_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ActivityIdError::NotAValidId(invalid_id.to_string())
        );
    }

    #[test]
    fn activity_new_should_create_activity_with_random_id() {
        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date");
        let start_time = NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time");
        let accounting_category_id = AccountingCategoryId::new();
        let task = "Test Task".to_string();

        let activity = Activity::new(
            date,
            start_time,
            accounting_category_id.clone(),
            task.clone(),
        );

        assert!(activity.id().0.is_nil() == false);
        assert_eq!(activity.date(), &date);
        assert_eq!(activity.start_time(), &start_time);
        assert_eq!(activity.accounting_category_id(), &accounting_category_id);
        assert_eq!(activity.task(), task);
    }

    #[test]
    fn activity_with_id_should_create_activity_with_specific_id() {
        let id = ActivityId::new();
        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date");
        let start_time = NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time");
        let accounting_category_id = AccountingCategoryId::new();
        let task = "Test Task".to_string();

        let activity = Activity::with_id(
            id.clone(),
            date,
            start_time,
            accounting_category_id.clone(),
            task.clone(),
        );

        assert_eq!(activity.id(), &id);
        assert_eq!(activity.date(), &date);
        assert_eq!(activity.start_time(), &start_time);
        assert_eq!(activity.accounting_category_id(), &accounting_category_id);
        assert_eq!(activity.task(), task);
    }

    #[test]
    fn duration_should_calculate_duration_with_valid_end_time() {
        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date");
        let start_time = NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time");
        let end_time = NaiveTime::from_hms_opt(10, 30, 0).expect("Valid activity end time");
        let accounting_category_id = AccountingCategoryId::new();
        let task = "Test Task".to_string();

        let mut activity = Activity::new(date, start_time, accounting_category_id, task);

        // Initially, end_time is None, so duration should be zero
        assert_eq!(activity.duration(), Duration::zero());

        // Set the end_time and check the duration
        activity.set_end_time(Some(end_time));
        assert_eq!(activity.duration(), Duration::minutes(90));
    }

    #[test]
    fn duration_should_return_zero_for_ongoing_activity() {
        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date");
        let start_time = NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time");
        let accounting_category_id = AccountingCategoryId::new();
        let task = "Test Task".to_string();

        let activity = Activity::new(date, start_time, accounting_category_id, task);

        // Since end_time is None, duration should be zero
        assert_eq!(activity.duration(), Duration::zero());
    }
}
