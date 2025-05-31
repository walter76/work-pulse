use super::activity::Activity;

/// Represents a list of activities.
/// 
/// It is used to record activities that the user did during his working day.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivitiesList {
    /// The list of activities.
    activities: Vec<Activity>,
}

impl ActivitiesList {
    /// Creates a new `ActivitiesList`.
    pub fn new() -> Self {
        Self {
            activities: vec![],
        }
    }

    /// Adds an activity to the list.
    /// 
    /// # Arguments
    /// 
    /// - `activity`: The activity to add.
    pub fn add_activity(&mut self, activity: Activity) {
        self.activities.push(activity);
    }

    /// Returns the list of activities.
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{activity::Activity, pam::PamCategoryId};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_add_activity() {
        let mut activities_list = ActivitiesList::new();
        let activity = Activity::new(
            NaiveDate::from_ymd(2023, 10, 1),
            NaiveTime::from_hms(9, 0, 0),
            PamCategoryId::new(),
            "Test Task".to_string(),
        );
        activities_list.add_activity(activity.clone());
        assert_eq!(activities_list.activities().len(), 1);
        assert_eq!(activities_list.activities()[0], activity);
    }
}