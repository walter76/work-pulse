use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
pub struct Category {
    id: String,
    name: String,
}

impl Category {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

const CATEGORY_SERVICE_URL: &str = "http://localhost:8080/api/v1/pam-categories";

pub struct CategoryService {
}

impl CategoryService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let response = reqwest::blocking::get(CATEGORY_SERVICE_URL)
            .with_context(|| format!("Failed to fetch PAM categories from {}", CATEGORY_SERVICE_URL))?;
    
        if response.status().is_success() {
            let pam_categories: Vec<Category> = response
                .json()
                .with_context(|| "Failed to parse PAM categories from response")?;
            Ok(pam_categories)
        } else {
            Err(anyhow::anyhow!(
                "Failed to fetch PAM categories: HTTP {}",
                response.status()
            ))
        }
    }
}