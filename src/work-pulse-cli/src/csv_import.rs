use std::{fs::File, io::{BufReader, Read}};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use encoding_rs::Encoding;
use serde::Deserialize;

use crate::{activity_service::ActivityService, category_mapper, category_service::CategoryService};

pub fn import(file_path: &str) -> Result<()> {
    println!("Importing CSV file: {}", file_path);

    let records = read_csv(file_path)?;

    for record in records.iter() {
        println!(
            "CW: {}, Date: {}, Check In: {}, Check Out: {}, PAM Category: {}, Task: {}, Comment: {}",
            record.cw,
            record.date,
            record.check_in,
            record.check_out,
            record.pam_category,
            record.task,
            record.comment
        );
    }

    check_and_create_pam_categories(&records)?;

    println!();
    println!("Categories from Service:");

    let categories_from_service = CategoryService::new().get_categories()?;
    for category in categories_from_service.iter() {
        println!("  {}: {}", category.id().unwrap(), category.name());
    }

    println!();
    println!("Creating Activities from CSV records...");
    println!("This might take a while, depending on the number of records in the CSV file.");

    let activity_service = ActivityService::new();

    for record in records.iter() {
        let date = convert_date_format(&record.date)
            .with_context(|| format!("Failed to convert date format for record: {}", record.date))?;

        let pam_category_id = categories_from_service
            .iter()
            .find(|c| c.name() == category_mapper::map_category(&record.pam_category).unwrap_or(&record.pam_category))
            .map(|c| c.id().unwrap())
            .unwrap()
            .to_string();

        let activity = activity_service.create_activity(
            date,
            record.check_in.clone(),
            Some(record.check_out.clone()),
            pam_category_id.clone(),
            record.task.clone(),
        )?;

        println!(
            "Created Activity: ID: {}, Date: {}, Start Time: {}, End Time: {}, PAM Category ID: {}, Task: {}",
            activity.id().unwrap_or("N/A"),
            activity.date(),
            activity.start_time(),
            activity.end_time().unwrap_or("N/A"),
            activity.pam_category_id(),
            activity.task()
        );
    }

    Ok(())
}

const ACTIVITIES_YEAR: &str = "2025";

fn convert_date_format(date: &str) -> Result<String> {
    // add the year of the activity
    let date = format!("{}{}", date, ACTIVITIES_YEAR);

    // Parse the date in "dd.mm.yyyy" format
    let parsed_date = NaiveDate::parse_from_str(&date, "%d.%m.%Y")?;

    // Format the date into "yyyy-mm-dd"
    Ok(parsed_date.format("%Y-%m-%d").to_string())
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

fn read_csv(file_path: &str) -> Result<Vec<ActivityTableRecord>> {
    // FIXME It might be required to detect the encoding of the CSV file, if we run that on Linux OS.

    let file = File::open(file_path)
        .with_context(|| format!("Failed to open CSV file: {}", file_path))?;
    let mut reader = BufReader::new(file);

    // read the raw bytes from the file
    let mut raw_bytes = Vec::new();
    reader
        .read_to_end(&mut raw_bytes)
        .with_context(|| format!("Failed to read CSV file: {}", file_path))?;
    
    // decode the bytes using latin-1 encoding
    let enc = Encoding::for_label(b"latin1")
        .with_context(|| "Failed to find encoding for latin1")?;
    let (decoded_content, _ , _) = enc.decode(&raw_bytes);

    let mut csv_reader = csv::Reader::from_reader(decoded_content.as_bytes());

    Ok(
        csv_reader
            .deserialize()
            .map(|result| result.with_context(|| "Failed to deserialize CSV record"))
            .collect::<Result<Vec<ActivityTableRecord>>>()?,
    )
}

fn check_and_create_pam_categories(records: &[ActivityTableRecord]) -> Result<()> {
    println!();
    println!("Checking PAM Categories against Service Categories:");

    let pam_categories = get_pam_categories(&records);
    let pam_categories_from_service = CategoryService::new().get_categories()?;
    for pam_category in pam_categories.iter() {
        let category_name = category_mapper::map_category(&pam_category)
            .unwrap_or(&pam_category);

        if !pam_categories_from_service.iter().any(|c| c.name() == category_name) {
            println!(
                "  {} -> {} (not found in service categories)",
                pam_category, category_name
            );

            CategoryService::new()
                .create_category(category_name)
                .with_context(|| format!("Failed to create category: {}", category_name))?;
        } else {
            println!("  {} -> {}", pam_category, category_name);
        }
    }

    Ok(())
}

fn get_pam_categories(records: &[ActivityTableRecord]) -> Vec<String> {
    let mut categories = records
        .iter()
        .map(|record| record.pam_category.clone())
        .collect::<Vec<String>>();

    categories.sort();
    categories.dedup();

    categories
}
