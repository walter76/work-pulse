use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime};
use thiserror::Error;
use uuid::Uuid;

use super::pam::PamCategoryId;

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
    /// Returns an error if an invalid UUID is provided.
    /// 
    /// # Arguments
    /// 
    /// - `s`: A string slice that represents a UUID.
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

    /// The PAM category ID associated with the activity.
    pam_category_id: PamCategoryId,

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
    /// - `pam_category_id`: The PAM category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    pub fn new(date: NaiveDate, start_time: NaiveTime, pam_category_id: PamCategoryId, task: String) -> Self {
        Self {
            id: ActivityId::new(),
            date,
            start_time,
            end_time: None,
            pam_category_id,
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
    /// - `pam_category_id`: The PAM category ID associated with the activity.
    /// - `task`: The task associated with the activity.
    pub fn with_id(id: ActivityId, date: NaiveDate, start_time: NaiveTime, pam_category_id: PamCategoryId, task: String) -> Self {
        Self { 
            id,
            date,
            start_time,
            end_time: None,
            pam_category_id,
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

    /// Returns the PAM category ID associated with the activity.
    pub fn pam_category_id(&self) -> &PamCategoryId {
        &self.pam_category_id
    }

    /// Sets the PAM category ID associated with the activity.
    /// 
    /// # Arguments
    /// 
    /// - `pam_category_id`: The PAM category ID to associate with the activity.
    pub fn set_pam_category_id(&mut self, pam_category_id: PamCategoryId) {
        self.pam_category_id = pam_category_id;
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
}
