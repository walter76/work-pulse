use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when working with `PamCategoryId`.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum PamCategoryIdError {
    /// The given string is not a valid PAM category id.
    #[error("The provided string is not a valid PAM category id: {0}")]
    NotAValidId(String),
}

/// The unique identifier for an PAM category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PamCategoryId(pub Uuid);

impl PamCategoryId {
    /// Creates a new `PamCategoryId` with a random UUID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a string into an `PamCategoryId`.
    /// 
    /// # Arguments
    /// 
    /// - `s`: A string slice that represents a UUID.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the `PamCategoryId` if successful, or a `PamCategoryIdError` if the string is not a valid UUID.
    pub fn parse_str(s: &str) -> Result<Self, PamCategoryIdError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| PamCategoryIdError::NotAValidId(s.to_string()))
    }
}

impl Display for PamCategoryId {
    /// Formats the `PamCategoryId` as a string.
    /// 
    /// # Arguments
    /// 
    /// - `f`: A mutable reference to a formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a category for PAM.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PamCategory {
    /// The unique identifier for the PAM category.
    id: PamCategoryId,

    /// The name of the PAM category.
    name: String,
}

impl PamCategory {
    /// Creates a new `PamCategory` with a random ID.
    /// 
    /// # Arguments
    /// 
    /// - `name`: The name of the PAM category.
    pub fn new(name: String) -> Self {
        Self {
            id: PamCategoryId::new(),
            name,
        }
    }

    /// Creates a new `PamCategory` with a specific ID.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The unique identifier for the PAM category.
    /// - `name`: The name of the PAM category.
    pub fn with_id(id: PamCategoryId, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the unique identifier of the PAM category.
    pub fn id(&self) -> &PamCategoryId {
        &self.id
    }

    /// Returns the name of the PAM category.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pam_category_id_new_should_create_pam_category_with_id() {
        let id = PamCategoryId::new();
        assert!(id.0.is_nil() == false);
    }

    #[test]
    fn pam_category_id_parse_str_should_parse_valid_id() {
        let unique_id = Uuid::new_v4();
        let pam_category_id = PamCategoryId::parse_str(unique_id.to_string().as_str()).unwrap();

        assert_eq!(unique_id, pam_category_id.0);
    }

    #[test]
    fn pam_category_id_parse_str_should_fail_with_invalid_id() {
        let invalid_id = "invalid-id";
        let result = PamCategoryId::parse_str(invalid_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamCategoryIdError::NotAValidId(invalid_id.to_string()));
    }

    #[test]
    fn pam_category_new_should_create_pam_category_with_name() {
        let category_name = "Test Category";
        let category = PamCategory::new(category_name.to_string());

        assert_eq!(category.name, category_name);
        assert!(category.id.0.is_nil() == false);
    }

    #[test]
    fn pam_category_with_id_should_create_pam_category_with_specific_id() {
        let unique_id = PamCategoryId::new();
        let category_name = "Test Category";
        let category = PamCategory::with_id(unique_id.clone(), category_name.to_string());

        assert_eq!(category.name, category_name);
        assert_eq!(category.id, unique_id);
    }
}