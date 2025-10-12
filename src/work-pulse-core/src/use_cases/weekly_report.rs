use chrono::{Duration, NaiveDate};

use crate::{adapters::ActivitiesListRepository, entities::{accounting::AccountingCategoryId, activity::Activity}};

/// A report summarizing activities for a specific week.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyReport {
    week_start: NaiveDate,
    activities: Vec<Activity>,
    total_duration: Duration,
    duration_per_category: Vec<(AccountingCategoryId, Duration)>,
}

// TODO: Implement the sum of durations for every day of the week.

impl WeeklyReport {
    /// Creates a new `WeeklyReport` for the week starting on `week_start`.
    ///
    /// # Arguments
    ///
    /// * `week_start` - The starting date of the week (should be a Monday).
    /// * `repository` - A reference to an implementation of `ActivitiesListRepository` to fetch activities.
    pub fn new(week_start: NaiveDate, repository: &dyn ActivitiesListRepository) -> Self {
        let week_end = week_start + Duration::days(7);
        let activities = repository.get_by_date_range(week_start, week_end);

        let total_duration = activities
            .iter()
            .map(|activity| activity.duration())
            .sum();

        let mut duration_per_category = Vec::new();
        let mut category_durations = std::collections::HashMap::new();

        for activity in &activities {
            let category_id = activity.accounting_category_id().clone();
            let duration = activity.duration();
            *category_durations.entry(category_id).or_insert(Duration::zero()) += duration;
        }

        for (category_id, duration) in category_durations {
            duration_per_category.push((category_id, duration));
        }

        WeeklyReport {
            week_start,
            activities,
            total_duration,
            duration_per_category,
        }
    }

    /// Returns the starting date of the week.
    pub fn week_start(&self) -> NaiveDate {
        self.week_start
    }

    /// Returns a slice of activities included in the report.
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// Returns the total duration of all activities in the report.
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// Returns a vector of tuples containing accounting category IDs and their corresponding total durations.
    pub fn duration_per_category(&self) -> &[(AccountingCategoryId, Duration)] {
        &self.duration_per_category
    }
}
