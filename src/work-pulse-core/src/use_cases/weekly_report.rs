use chrono::{Duration, NaiveDate};

use crate::{
    adapters::ActivitiesListRepository,
    entities::{accounting::AccountingCategoryId, activity::Activity},
};

/// A report summarizing activities for a specific week.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyReport {
    /// The starting date of the week (typically a Monday).
    week_start: NaiveDate,

    /// The ending date of the week (typically a Sunday).
    week_end: NaiveDate,

    /// The list of activities recorded during the week.
    activities: Vec<Activity>,

    /// The total duration of all activities recorded during the week.
    total_duration: Duration,

    /// A vector of tuples containing accounting category IDs and their corresponding total durations.
    duration_per_category: Vec<(AccountingCategoryId, Duration)>,

    /// A vector of tuples containing each day of the week and a vector of tuples with accounting category IDs and their corresponding total durations for that day.
    daily_durations_per_category: Vec<(NaiveDate, Vec<(AccountingCategoryId, Duration)>)>,
}

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

        let total_duration = activities.iter().map(|activity| activity.duration()).sum();

        let mut duration_per_category = Vec::new();
        let mut category_durations = std::collections::HashMap::new();

        for activity in &activities {
            let category_id = activity.accounting_category_id().clone();
            let duration = activity.duration();
            *category_durations
                .entry(category_id)
                .or_insert(Duration::zero()) += duration;
        }

        for (category_id, duration) in category_durations {
            duration_per_category.push((category_id, duration));
        }

        let daily_durations_per_category =
            Self::calculate_daily_durations_per_category(&activities, week_start);

        WeeklyReport {
            week_start,
            week_end,
            activities,
            total_duration,
            daily_durations_per_category,
            duration_per_category,
        }
    }

    fn calculate_daily_durations_per_category(
        activities: &[Activity],
        week_start: NaiveDate,
    ) -> Vec<(NaiveDate, Vec<(AccountingCategoryId, Duration)>)> {
        let mut daily_durations = Vec::new();

        // iterate through each day of the week
        for day_offset in 0..7 {
            let current_date = week_start + Duration::days(day_offset);

            // group activities by category for this specific date
            let mut daily_category_durations = std::collections::HashMap::new();

            for activity in activities.iter().filter(|a| *a.date() == current_date) {
                let category_id = activity.accounting_category_id().clone();
                let duration = activity.duration();
                *daily_category_durations
                    .entry(category_id)
                    .or_insert(Duration::zero()) += duration;
            }

            // Convert the daily category durations into a vector and add it to the daily durations
            daily_durations.push((current_date, daily_category_durations.into_iter().collect()));
        }

        daily_durations
    }

    /// Returns the starting date (Monday) of the week for the report.
    pub fn week_start(&self) -> NaiveDate {
        self.week_start
    }

    /// Returns the ending date (Sunday) of the week for the report.
    pub fn week_end(&self) -> NaiveDate {
        self.week_end
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

    /// Returns a vector of tuples containing each day of the week and a vector of tuples with accounting category IDs and their corresponding total durations for that day.
    pub fn daily_durations_per_category(
        &self,
    ) -> &[(NaiveDate, Vec<(AccountingCategoryId, Duration)>)] {
        &self.daily_durations_per_category
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
    async fn weekly_report_should_aggregate_activities_and_calculate_total_duration() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository.clone());

        let _activity1 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 2).expect("Valid activity date"), // Monday
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Activity 1".to_string(),
            )
            .await;

        let _activity2 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 4).expect("Valid activity date"), // Wednesday
                NaiveTime::from_hms_opt(11, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(12, 30, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Activity 2".to_string(),
            )
            .await;

        let _activity3 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 6).expect("Valid activity date"), // Friday
                NaiveTime::from_hms_opt(14, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(15, 15, 0).expect("Valid activity end time")),
                AccountingCategoryId::new(),
                "Activity 3".to_string(),
            )
            .await;

        let report = WeeklyReport::new(
            NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(), // Start of the week (Monday)
            &*repository.lock().await,
        );

        assert_eq!(
            report.week_start(),
            NaiveDate::from_ymd_opt(2023, 10, 2).unwrap()
        );
        assert_eq!(
            report.week_end(),
            NaiveDate::from_ymd_opt(2023, 10, 9).unwrap()
        );
        assert_eq!(report.activities().len(), 3);
        assert_eq!(
            report.total_duration(),
            Duration::hours(3) + Duration::minutes(45)
        );
    }

    #[tokio::test]
    async fn weekly_report_with_no_activities_should_have_zero_duration() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));

        let report = WeeklyReport::new(
            NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(), // Start of the week (Monday)
            &*repository.lock().await,
        );

        assert_eq!(
            report.week_start(),
            NaiveDate::from_ymd_opt(2023, 10, 2).unwrap()
        );
        assert_eq!(
            report.week_end(),
            NaiveDate::from_ymd_opt(2023, 10, 9).unwrap()
        );
        assert_eq!(report.activities().len(), 0);
        assert_eq!(report.total_duration(), Duration::zero());
    }

    #[tokio::test]
    async fn weekly_report_should_calculate_duration_per_category() {
        let repository = Arc::new(Mutex::new(InMemoryActivitiesListRepository::new()));
        let mut activities_list = ActivitiesList::new(repository.clone());

        let category1 = AccountingCategoryId::new();
        let category2 = AccountingCategoryId::new();

        let _activity1 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 2).expect("Valid activity date"), // Monday
                NaiveTime::from_hms_opt(9, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(10, 0, 0).expect("Valid activity end time")),
                category1.clone(),
                "Activity 1".to_string(),
            )
            .await;

        let _activity2 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 4).expect("Valid activity date"), // Wednesday
                NaiveTime::from_hms_opt(11, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(12, 30, 0).expect("Valid activity end time")),
                category2.clone(),
                "Activity 2".to_string(),
            )
            .await;

        let _activity3 = activities_list
            .record(
                NaiveDate::from_ymd_opt(2023, 10, 6).expect("Valid activity date"), // Friday
                NaiveTime::from_hms_opt(14, 0, 0).expect("Valid activity start time"),
                Some(NaiveTime::from_hms_opt(15, 15, 0).expect("Valid activity end time")),
                category2.clone(),
                "Activity 3".to_string(),
            )
            .await;

        let report = WeeklyReport::new(
            NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(), // Start of the week (Monday)
            &*repository.lock().await,
        );

        let mut duration_map = std::collections::HashMap::new();
        for (category_id, duration) in report.duration_per_category() {
            duration_map.insert(category_id.clone(), *duration);
        }

        assert_eq!(duration_map.get(&category1), Some(&Duration::hours(1)));
        assert_eq!(
            duration_map.get(&category2),
            Some(&(Duration::hours(2) + Duration::minutes(45)))
        );
    }
}
