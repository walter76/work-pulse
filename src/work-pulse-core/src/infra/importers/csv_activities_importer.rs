use std::{io::Read, sync::Arc};

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use csv::Reader;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{
    adapters::{AccountingCategoriesListRepository, ActivitiesImporter, ActivitiesImporterError},
    entities::activity::Activity,
};

/// An importer for activities from CSV files.
pub struct CsvActivitiesImporter {
    /// The repository for managing accounting categories.
    accounting_categories_list_repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
}

impl CsvActivitiesImporter {
    /// Creates a new `CsvActivitiesImporter`.
    ///
    /// # Arguments
    ///
    /// - `accounting_categories_list_repository`: An `Arc<Mutex<dyn AccountingCategoriesListRepository>>` to manage accounting categories.
    pub fn new(
        accounting_categories_list_repository: Arc<Mutex<dyn AccountingCategoriesListRepository>>,
    ) -> Self {
        Self {
            accounting_categories_list_repository,
        }
    }
}

#[async_trait]
impl ActivitiesImporter for CsvActivitiesImporter {
    /// Imports activities from a CSV reader for a specific year.
    ///
    /// # Arguments
    ///
    /// - `reader`: A reader that provides the CSV data.
    /// - `year`: The year to associate with the imported activities.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Activity>)` if the import is successful.
    /// - `Err(ActivitiesImporterError)` if there is an error during import.
    async fn import<R>(
        &mut self,
        reader: R,
        year: u16,
    ) -> Result<Vec<Activity>, ActivitiesImporterError>
    where
        R: Read + Send,
    {
        let mut csv_reader = Reader::from_reader(reader);
        let mut records = Vec::new();

        for result in csv_reader.deserialize() {
            let record: ActivityTableRecord =
                result.map_err(|_| ActivitiesImporterError::ParseError)?;
            records.push(record);
        }

        let mut activities = Vec::new();
        let mut accounting_categories_list_repository =
            self.accounting_categories_list_repository.lock().await;

        for activity_record in records {
            let date =
                ActivityTableRecord::convert_date_format(&activity_record.date, &year.to_string())?;

            let accounting_category = accounting_categories_list_repository
                .get_or_create_by_name(&activity_record.pam_category)
                .await
                .map_err(|_| ActivitiesImporterError::ParseError)?;

            let mut activity = Activity::new(
                date.parse()
                    .map_err(|_| ActivitiesImporterError::ParseError)?,
                activity_record
                    .check_in
                    .parse::<NaiveTime>()
                    .map_err(|_| ActivitiesImporterError::ParseError)?,
                accounting_category.id().clone(),
                activity_record.task,
            );

            let end_time = activity_record
                .check_out
                .parse::<NaiveTime>()
                .map_err(|_| ActivitiesImporterError::ParseError)?;
            activity.set_end_time(Some(end_time));

            activities.push(activity);
        }

        Ok(activities)
    }
}

/// A record representing a row in the activity CSV file.
/// The fields correspond to the columns in the CSV file.
#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
struct ActivityTableRecord {
    #[serde(rename = "CW")]
    pub cw: u8,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Check In")]
    pub check_in: String,

    #[serde(rename = "Check Out")]
    pub check_out: String,

    #[serde(rename = "PAM Category")]
    pub pam_category: String,

    #[serde(rename = "Topic")]
    pub task: String,

    #[serde(rename = "Comment")]
    pub comment: String,
}

impl ActivityTableRecord {
    /// Converts a date string from "dd.mm." format to "yyyy-mm-dd" format by adding the specified year.
    ///
    /// # Arguments
    ///
    /// - `date`: A string slice representing the date in "dd.mm." format.
    /// - `year`: A string slice representing the year to be added.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` containing the date in "yyyy-mm-dd" format if successful.
    /// - `Err(ActivitiesImporterError)` if the date cannot be parsed.
    fn convert_date_format(date: &str, year: &str) -> Result<String, ActivitiesImporterError> {
        // add the year of the activity
        let date = format!("{}{}", date, year);

        // Parse the date in "dd.mm.yyyy" format
        let parsed_date = NaiveDate::parse_from_str(&date, "%d.%m.%Y")
            .map_err(|_| ActivitiesImporterError::ParseError)?;

        // Format the date into "yyyy-mm-dd"
        Ok(parsed_date.format("%Y-%m-%d").to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repositories::in_memory::accounting_categories_list::InMemoryAccountingCategoriesListRepository;

    use super::*;

    #[test]
    fn convert_date_format_should_convert_valid_date() {
        let date = "15.03.";
        let year = "2023";
        let expected = "2023-03-15";

        let result = ActivityTableRecord::convert_date_format(date, year).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_date_format_should_fail_with_invalid_date() {
        let date = "31.02."; // Invalid date
        let year = "2023";

        let result = ActivityTableRecord::convert_date_format(date, year);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ActivitiesImporterError::ParseError);
    }

    #[tokio::test]
    async fn import_should_import_activities_from_csv() {
        let csv_data = "\
CW,Date,Check In,Check Out,PAM Category,Topic,Comment
11,15.03.,09:00,17:00,Development,Coding,Worked on project X
11,16.03.,10:00,18:00,Meeting,Team Meeting,Discussed project Y
";

        let reader = csv_data.as_bytes();
        let accounting_repo =
            Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut importer = CsvActivitiesImporter::new(accounting_repo);

        let activities = importer.import(reader, 2023).await.unwrap();
        assert_eq!(activities.len(), 2);

        assert_eq!(activities[0].date().to_string(), "2023-03-15");
        assert_eq!(activities[0].start_time().to_string(), "09:00:00");
        assert_eq!(activities[0].end_time().unwrap().to_string(), "17:00:00");
        assert_eq!(activities[0].task(), "Coding");

        assert_eq!(activities[1].date().to_string(), "2023-03-16");
        assert_eq!(activities[1].start_time().to_string(), "10:00:00");
        assert_eq!(activities[1].end_time().unwrap().to_string(), "18:00:00");
        assert_eq!(activities[1].task(), "Team Meeting");
    }

    #[tokio::test]
    async fn import_should_fail_with_invalid_csv() {
        let csv_data = "\
CW,Date,Check In,Check Out,PAM Category,Topic,Comment
11,invalid-date,09:00,17:00,Development,Coding,Worked on project X
";
        let reader = csv_data.as_bytes();
        let accounting_repo =
            Arc::new(Mutex::new(InMemoryAccountingCategoriesListRepository::new()));
        let mut importer = CsvActivitiesImporter::new(accounting_repo);

        let result = importer.import(reader, 2023).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ActivitiesImporterError::ParseError);
    }
}
