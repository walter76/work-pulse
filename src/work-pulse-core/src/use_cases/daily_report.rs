use chrono::{Duration, NaiveDate};

use crate::{adapters::ActivitiesListRepository, entities::activity::Activity};

/// Represents a daily report containing activities and total duration for a specific date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyReport {
    date: NaiveDate,
    activities: Vec<Activity>,
    total_duration: Duration,
}

impl DailyReport {
    pub fn new(date: NaiveDate, repository: &dyn ActivitiesListRepository) -> Self {
        let activities = repository.get_by_date(date);

        let total_duration = activities
            .iter()
            .map(|activity| Self::calculate_activity_duration(activity))
            .sum();

        DailyReport {
            date,
            activities,
            total_duration,
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    fn calculate_activity_duration(activity: &Activity) -> Duration {
        if let Some(end_time) = activity.end_time() {
            *end_time - *activity.start_time()
        } else {
            Duration::zero()
        }
    }
}
