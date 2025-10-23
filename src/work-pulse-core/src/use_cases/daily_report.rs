use chrono::{Duration, NaiveDate};

use crate::{adapters::ActivitiesListRepository, entities::activity::Activity};

/// Represents a daily report containing activities and total duration for a specific date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyReport {
    /// The date of the report. Also represents the date of the activities used to generate the report.
    date: NaiveDate,

    /// The list of activities recorded for the report date.
    activities: Vec<Activity>,

    /// The total duration of all activities recorded for the report date.
    total_duration: Duration,
}

impl DailyReport {
    /// Creates a new `DailyReport` for the specified date using the provided repository.
    ///
    /// # Arguments
    ///
    /// - `date`: The date for which the report is generated.
    /// - `repository`: The repository used to fetch activities for the specified date.
    pub async fn new(date: NaiveDate, repository: &dyn ActivitiesListRepository) -> Self {
        let activities = repository.get_by_date(date).await;

        let total_duration = activities.iter().map(|activity| activity.duration()).sum();

        DailyReport {
            date,
            activities,
            total_duration,
        }
    }

    /// Returns the date of the report.
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Returns the list of activities recorded for the report date.
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// Returns the total duration of all activities recorded for the report date.
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveTime;
    use tokio::sync::Mutex;

    use crate::{
        entities::accounting::AccountingCategoryId,
        infra::repositories::in_memory::activities_list::InMemoryActivitiesListRepository,
        use_cases::activities_list::ActivitiesList,
    };

    use super::*;

    #[tokio::test]
    async fn daily_report_should_aggregate_activities_and_total_duration() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository.clone());

        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid activity date");

        let activity1 = activities_list
            .record(
                date,
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Task 1".to_string(),
            )
            .await;

        let activity2 = activities_list
            .record(
                date,
                NaiveTime::from_hms_opt(11, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(12, 30, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Task 2".to_string(),
            )
            .await;

        let daily_report = DailyReport::new(date, &*repository.lock().await).await;

        assert_eq!(daily_report.date(), date);
        assert_eq!(
            daily_report.activities(),
            &[activity1.clone(), activity2.clone()]
        );
        assert_eq!(
            daily_report.total_duration(),
            Duration::minutes(90) + Duration::minutes(60)
        );
    }
}
