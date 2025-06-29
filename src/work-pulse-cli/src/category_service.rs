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
    client: reqwest::blocking::Client,
    base_url: String,
}

impl Default for CategoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl CategoryService {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: CATEGORY_SERVICE_URL.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let response = self.client.get(&self.base_url)
            .send()
            .with_context(|| format!("Failed to fetch PAM categories from {}", self.base_url))?;

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
        let response = self.client.post(&self.base_url)
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