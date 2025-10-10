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

    pub fn calculate_activity_duration(activity: &Activity) -> Duration {
        if let Some(end_time) = activity.end_time() {
            *end_time - *activity.start_time()
        } else {
            Duration::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::NaiveTime;

    use crate::{
        entities::accounting::AccountingCategoryId,
        infra::repositories::in_memory::activities_list::InMemoryActivitiesListRepository,
        use_cases::activities_list::ActivitiesList,
    };

    use super::*;

    #[test]
    fn calculate_activity_duration_should_return_correct_duration() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let start_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let end_time = NaiveTime::from_hms_opt(10, 30, 0).unwrap();

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            start_time,
            Some(end_time),
            AccountingCategoryId::new(),
            "Test Task".to_string(),
        );

        let duration = DailyReport::calculate_activity_duration(&activity);
        assert_eq!(duration, Duration::minutes(90));
    }

    #[test]
    fn calculate_activity_duration_should_return_zero_for_ongoing_activity() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository);

        let start_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

        let activity = activities_list.record(
            NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date"),
            start_time,
            None,
            AccountingCategoryId::new(),
            "Ongoing Task".to_string(),
        );

        let duration = DailyReport::calculate_activity_duration(&activity);
        assert_eq!(duration, Duration::zero());
    }

    #[test]
    fn daily_report_should_aggregate_activities_and_total_duration() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository.clone());

        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Valid date");

        let activity1 = activities_list.record(
            date,
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            Some(NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            AccountingCategoryId::new(),
            "Task 1".to_string(),
        );

        let activity2 = activities_list.record(
            date,
            NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            Some(NaiveTime::from_hms_opt(12, 30, 0).unwrap()),
            AccountingCategoryId::new(),
            "Task 2".to_string(),
        );

        let daily_report = DailyReport::new(date, &*repository.lock().unwrap());

        assert_eq!(daily_report.date(), date);
        assert_eq!(daily_report.activities(), &[activity1.clone(), activity2.clone()]);
        assert_eq!(
            daily_report.total_duration(),
            Duration::minutes(90) + Duration::minutes(60)
        );
    }
}
