use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    id: Option<String>,
    date: String,
    start_time: String,
    end_time: Option<String>,
    pam_category_id: String,
    task: String,
}

impl Activity {
    pub fn new(date: String, start_time: String, end_time: Option<String>, pam_category_id: String, task: String) -> Self {
        Self { 
            id: None,
            date,
            start_time,
            end_time,
            pam_category_id,
            task,
        }
    }

    pub fn with_id(id: String, date: String, start_time: String, end_time: Option<String>, pam_category_id: String, task: String) -> Self {
        Self {
            id: Some(id),
            date,
            start_time,
            end_time,
            pam_category_id,
            task,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn start_time(&self) -> &str {
        &self.start_time
    }

    pub fn end_time(&self) -> Option<&str> {
        self.end_time.as_deref()
    }

    pub fn pam_category_id(&self) -> &str {
        &self.pam_category_id
    }

    pub fn task(&self) -> &str {
        &self.task
    }
}

const ACTIVITY_SERVICE_URL: &str = "http://localhost:8080/api/v1/activities";

pub struct ActivityService {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl Default for ActivityService {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityService {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: ACTIVITY_SERVICE_URL.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn create_activity(&self, date: String, start_time: String, end_time: Option<String>, pam_category_id: String, task: String) -> Result<Activity> {
        let response = self.client.post(&self.base_url)
            .json(&Activity::new(date.clone(), start_time.clone(), end_time.clone(), pam_category_id.clone(), task.clone()))
            .send()
            .with_context(|| format!("Failed to create activity: date={}, start_time={}, end_time={:?}, pam_category_id={}, task={}", date, start_time, end_time, pam_category_id, task))?;

        if response.status().is_success() {
            let created_activity: Activity = response
                .json()
                .with_context(|| "Failed to parse created activity from response")?;
            Ok(created_activity)
        } else {
            Err(anyhow::anyhow!(
                "Failed to create activity: HTTP {}",
                response.status()
            ))
        }
    }
}
