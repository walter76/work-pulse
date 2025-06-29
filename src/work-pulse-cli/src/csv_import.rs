use std::{fs::File, io::{BufReader, Read}};

use anyhow::{Context, Result};
use encoding_rs::Encoding;
use serde::Deserialize;

use crate::{category_mapper, category_service::CategoryService};

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
    for category in categories_from_service {
        println!("  {}: {}", category.id().unwrap(), category.name());
    }

    Ok(())
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
