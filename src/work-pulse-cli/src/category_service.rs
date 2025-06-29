use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Category {
    id: Option<String>,
    name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
        }
    }

    #[allow(dead_code)]
    pub fn with_id(id: String, name: String) -> Self {
        Self {
            id: Some(id),
            name,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
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

    pub fn create_category(&self, category_name: &str) -> Result<Category> {
        let client = reqwest::blocking::Client::new();
        let response = client.post(CATEGORY_SERVICE_URL)
            .json(&Category::new(category_name.to_string()))
            .send()
            .with_context(|| format!("Failed to create category: {}", category_name))?;

        if response.status().is_success() {
            let created_category: Category = response
                .json()
                .with_context(|| "Failed to parse created category from response")?;
            Ok(created_category)
        } else {
            Err(anyhow::anyhow!(
                "Failed to create category: HTTP {}",
                response.status()
            ))
        }
    }
}