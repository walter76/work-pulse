use std::{fs::File, io::{BufReader, Read}};

use anyhow::{Context, Result};
use encoding_rs::Encoding;
use serde::Deserialize;

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

    println!();
    println!("Unique PAM Categories:");

    let pam_categories = get_pam_categories(&records);
    for pam_category in pam_categories {
        println!("  {}", pam_category);
    }

    println!();
    println!("PAM Categories from Service:");

    let pam_categories_from_service = get_pam_categories_from_service()?;
    for pam_category in pam_categories_from_service {
        println!("  {}: {}", pam_category.id, pam_category.name);
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

fn get_pam_categories(records: &[ActivityTableRecord]) -> Vec<String> {
    let mut categories = records
        .iter()
        .map(|record| record.pam_category.clone())
        .collect::<Vec<String>>();

    categories.sort();
    categories.dedup();

    categories
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
struct PamCategory {
    id: String,
    name: String,
}

fn get_pam_categories_from_service() -> Result<Vec<PamCategory>> {
    let url = "http://localhost:8080/api/v1/pam-categories";
    
    let response = reqwest::blocking::get(url)
        .with_context(|| format!("Failed to fetch PAM categories from {}", url))?;

    if response.status().is_success() {
        let pam_categories: Vec<PamCategory> = response
            .json()
            .with_context(|| "Failed to parse PAM categories from response")?;
        Ok(pam_categories)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch PAM categories: HTTP {}",
            response.status()
        ))
    }
}
