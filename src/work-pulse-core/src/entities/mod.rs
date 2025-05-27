use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

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
}

impl Activity {
    /// Creates a new activity.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier for the activity.
    pub fn new(id: ActivityId) -> Self {
        Self { id }
    }

    /// Returns the unique identifier for the activity.
    pub fn id(&self) -> &ActivityId {
        &self.id
    }
}
