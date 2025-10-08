use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when working with `AccountingCategoryId`.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum AccountingCategoryIdError {
    /// The given string is not a valid accounting category id.
    #[error("The provided string is not a valid accounting category id: {0}")]
    NotAValidId(String),
}

/// The unique identifier for an accounting category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountingCategoryId(pub Uuid);

impl AccountingCategoryId {
    /// Creates a new `AccountingCategoryId` with a random UUID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a string into an `AccountingCategoryId`.
    ///
    /// # Arguments
    ///
    /// - `s`: A string slice that represents a UUID.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AccountingCategoryId` if successful, or a `AccountingCategoryIdError` if the string is not a valid UUID.
    pub fn parse_str(s: &str) -> Result<Self, AccountingCategoryIdError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| AccountingCategoryIdError::NotAValidId(s.to_string()))
    }
}

impl Display for AccountingCategoryId {
    /// Formats the `AccountingCategoryId` as a string.
    ///
    /// # Arguments
    ///
    /// - `f`: A mutable reference to a formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a category for accounting.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountingCategory {
    /// The unique identifier for the accounting category.
    id: AccountingCategoryId,

    /// The name of the accounting category.
    name: String,
}

impl AccountingCategory {
    /// Creates a new `AccountingCategory` with a random ID.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the accounting category.
    pub fn new(name: String) -> Self {
        Self {
            id: AccountingCategoryId::new(),
            name,
        }
    }

    /// Creates a new `AccountingCategory` with a specific ID.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier for the accounting category.
    /// - `name`: The name of the accounting category.
    pub fn with_id(id: AccountingCategoryId, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the unique identifier of the accounting category.
    pub fn id(&self) -> &AccountingCategoryId {
        &self.id
    }

    /// Returns the name of the accounting category.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the accounting category.
    ///
    /// # Arguments
    ///
    /// - `name`: The new name for the accounting category.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounting_category_id_new_should_create_accounting_category_with_id() {
        let id = AccountingCategoryId::new();
        assert!(id.0.is_nil() == false);
    }

    #[test]
    fn accounting_category_id_parse_str_should_parse_valid_id() {
        let unique_id = Uuid::new_v4();
        let accounting_category_id =
            AccountingCategoryId::parse_str(unique_id.to_string().as_str()).unwrap();

        assert_eq!(unique_id, accounting_category_id.0);
    }

    #[test]
    fn accounting_category_id_parse_str_should_fail_with_invalid_id() {
        let invalid_id = "invalid-id";
        let result = AccountingCategoryId::parse_str(invalid_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountingCategoryIdError::NotAValidId(invalid_id.to_string())
        );
    }

    #[test]
    fn accounting_category_new_should_create_accounting_category_with_name() {
        let category_name = "Test Category";
        let category = AccountingCategory::new(category_name.to_string());

        assert_eq!(category.name, category_name);
        assert!(category.id.0.is_nil() == false);
    }

    #[test]
    fn accounting_category_with_id_should_create_accounting_category_with_specific_id() {
        let unique_id = AccountingCategoryId::new();
        let category_name = "Test Category";
        let category = AccountingCategory::with_id(unique_id.clone(), category_name.to_string());

        assert_eq!(category.name, category_name);
        assert_eq!(category.id, unique_id);
    }

    #[test]
    fn accounting_category_set_name_should_update_category_name() {
        let mut category = AccountingCategory::new("Initial Name".to_string());
        let new_name = "Updated Name".to_string();
        category.set_name(new_name.clone());

        assert_eq!(category.name, new_name);
    }
}
