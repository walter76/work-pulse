use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime};
use thiserror::Error;
use uuid::Uuid;

// There are two use cases for the activity:
//   1. The user starts the activity and then stops it later.
//   2. The user records an activity that was done in the past.
// How to model the two use cases in the domain with entities?

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
}

impl Activity {
    /// Creates a new activity.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier for the activity.
    /// - `date`: The date when the activity was performed.
    /// - `start_time`: The time when the activity started.
    pub fn new(id: ActivityId, date: NaiveDate, start_time: NaiveTime) -> Self {
        Self { id, date, start_time }
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
}


/// Represents a list of activities.
/// 
/// It is used to record activities that the user did during his working day.
pub struct ActivitiesList;

/// Represents a tracker for activities.
/// 
/// It is used to track an activity that the user is currently doing. Supports
/// starting, stopping, suspending, and resuming activities.
/// 
/// The ActivityTracker is recording the activity after it is finished in the
/// ActivitiesList.
pub struct ActivityTracker;
