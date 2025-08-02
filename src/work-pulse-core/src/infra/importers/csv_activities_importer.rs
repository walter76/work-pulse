use std::io::Read;

use chrono::{NaiveDate, NaiveTime};
use csv::Reader;
use serde::Deserialize;

use crate::{adapters::{ActivitiesImporter, ActivitiesImporterError}, entities::{activity::Activity, pam::PamCategoryId}};

pub struct CsvActivitiesImporter<R: Read> {
    reader: R,
    activities_year: String,
}

impl<R: Read> CsvActivitiesImporter<R> {
    pub fn new(reader: R, activities_year: String) -> Self {
        CsvActivitiesImporter { reader, activities_year }
    }
}

impl<R: Read> ActivitiesImporter for CsvActivitiesImporter<R> {
    fn import(&mut self) -> Result<Vec<Activity>, ActivitiesImporterError> {
        let mut csv_reader = Reader::from_reader(&mut self.reader);
        let mut records = Vec::new();

        for result in csv_reader.deserialize() {
            let record: ActivityTableRecord = result.map_err(|_| ActivitiesImporterError::ParseError)?;
            records.push(record);
        }

        let mut activities = Vec::new();

        for activity_record in records {
            let date = ActivityTableRecord::convert_date_format(&activity_record.date, &self.activities_year)?;

            let mut activity = Activity::new(
                date.parse()
                    .map_err(|_| ActivitiesImporterError::ParseError)?,
                activity_record.check_in.parse::<NaiveTime>()
                    .map_err(|_| ActivitiesImporterError::ParseError)?,
                PamCategoryId::parse_str(&activity_record.pam_category)
                    .map_err(|_| ActivitiesImporterError::ParseError)?,
                activity_record.task,
            );

            let end_time = activity_record.check_out.parse::<NaiveTime>().map_err(|_| ActivitiesImporterError::ParseError)?;
            activity.set_end_time(Some(end_time));

            activities.push(activity);
        }

        Ok(activities)
    }
}

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
