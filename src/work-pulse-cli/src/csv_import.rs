use std::{fs::File, io::{BufReader, Read}};

use anyhow::{Context, Result};
use serde::Deserialize;

pub fn import(file_path: &str) -> Result<()> {
    println!("Importing CSV file: {}", file_path);

    let records = read_csv(file_path)?;

    for record in records {
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
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open CSV file: {}", file_path))?;
    let mut reader = BufReader::new(file);

    // read the raw bytes from the file
    let mut raw_bytes = Vec::new();
    reader
        .read_to_end(&mut raw_bytes)
        .with_context(|| format!("Failed to read CSV file: {}", file_path))?;
    
    // decode the bytes using WINDOWS-1252 encoding
    let (decoded_content, _ , _) = encoding_rs::WINDOWS_1252.decode(&raw_bytes);


    let mut csv_reader = csv::Reader::from_reader(decoded_content.as_bytes());

    Ok(
        csv_reader
            .deserialize()
            .map(|result| result.with_context(|| "Failed to deserialize CSV record"))
            .collect::<Result<Vec<ActivityTableRecord>>>()?,
    )
}
