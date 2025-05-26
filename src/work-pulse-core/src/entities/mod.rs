use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when working with `BookingId`.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum BookingIdError {
    /// The id is not a valid booking id.
    #[error("The provided id is not a valid booking id: {0}")]
    NotAValidId(String),
}

/// The unique identifier for a booking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BookingId(pub Uuid);

impl BookingId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse_str(s: &str) -> Result<Self, BookingIdError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| BookingIdError::NotAValidId(s.to_string()))
    }
}

impl Display for BookingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a booking in the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Booking {
    /// The unique identifier for the booking.
    id: BookingId,
}

impl Booking {
    /// Creates a new booking.
    pub fn new(id: BookingId) -> Self {
        Self { id }
    }

    /// Returns the unique identifier for the booking.
    pub fn id(&self) -> &BookingId {
        &self.id
    }
}
