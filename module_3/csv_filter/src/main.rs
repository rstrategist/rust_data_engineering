use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct VixRecord {
    #[serde(rename = "DATE")]
    date: String,
    #[serde(rename = "VIXCLS")]
    value: Option<f64>, // Use Option to handle missing values
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = "data/VIXCLS.csv";
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Print the headers
    let headers = rdr.headers()?;
    println!("CSV Headers: {:?}", headers);

    // Keep track of record numbers for summary stats
    let mut filtered_records = Vec::new();
    let mut total_records = 0;
    let mut skipped_records = 0;

    // Iterate over records
    for result in rdr.deserialize() {
        total_records += 1;
        match result {
            Ok(record) => {
                let record: VixRecord = record;
                // Check if the value is present and meets the condition
                if let Some(value) = record.value {
                    if value > 20.0 {
                        filtered_records.push(record);
                    }
                } else {
                    skipped_records += 1;
                }
            }
            Err(e) => {
                println!("Error deserializing record: {:?}", e);
                skipped_records += 1;
            }
        }
    }

    // Write filtered records to a new CSV file
    let mut wtr = csv::Writer::from_writer(File::create("filtered_vix.csv")?);
    for record in &filtered_records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    // Print summary statistics
    println!("Summary Statistics:");
    println!("Total records parsed: {}", total_records);
    println!("Records skipped: {}", skipped_records);
    println!("Records in the filter: {}", filtered_records.len());

    Ok(())
}
