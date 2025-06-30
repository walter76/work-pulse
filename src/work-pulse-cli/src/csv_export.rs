use anyhow::Result;

pub fn export(file_path: &str) -> Result<()> {
    println!("Exporting activities to CSV file: {}", file_path);

    Ok(())
}
