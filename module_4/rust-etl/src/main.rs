// ETL Example
/*
This program demonstrates an ETL (Extract, Transform, Load) process in Rust.
It takes raw data, transforms it by capping outliers (values > 100) and
correcting negatives (values < 0), and then saves the cleaned data into a
CSV file. The program also provides summary statistics (total and average
of cleaned values) and includes unit tests to validate the process.
Error handling is implemented to ensure robustness during data transformation
and file operations.
*/

use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct RawData {
    id: u32,
    value: i32,
}

#[derive(Debug)]
struct CleanData {
    id: u32,
    value: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = vec![
        RawData { id: 1, value: 1 },
        RawData { id: 2, value: -5 },
        RawData { id: 3, value: 150 }, // Example of an outlier
        RawData { id: 4, value: 50 },
    ];

    match extract_transform_load(raw) {
        Ok(cleaned) => {
            // Print cleaned data
            for data in &cleaned {
                println!("Clean Data: Id - {:?} Value - {:?}", data.id, data.value);
            }

            // Summarise the cleaned data
            let total: i32 = cleaned.iter().map(|data| data.value).sum();
            let average: f32 = total as f32 / cleaned.len() as f32;

            println!("\nSummary:");
            println!("Total: {}", total);
            println!("Average: {:.2}", average);

            // Save cleaned data to csv
            save_to_csv(&cleaned, "cleaned_data.csv")?;
        }
        Err(e) => {
            eprintln!("Error during ETL process: {}", e);
        }
    }

    Ok(())
}

// Perform ETL process
fn extract_transform_load(raw: Vec<RawData>) -> Result<Vec<CleanData>, String> {
    let mut cleaned_data = Vec::new();

    for r in raw {
        // Handle potential errors in transformation
        let transformed_value = transform_value(r.value)?;
        cleaned_data.push(CleanData {
            id: r.id,
            value: transformed_value,
        });
    }

    Ok(cleaned_data)
}

// Transform the value with validation
fn transform_value(value: i32) -> Result<i32, String> {
    if value < 0 {
        Ok(0) // Negative values are set to zero
    } else if value > 100 {
        Ok(100) // Values over 100 are capped at 100
    } else {
        Ok(value) // Valid value
    }
}

// Save cleaned data to a CSV file
fn save_to_csv(cleaned_data: &[CleanData], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "id,value")?; // Write CSV header
    for data in cleaned_data {
        writeln!(file, "{},{}", data.id, data.value)?; // Write each row
    }
    println!("\nCleaned data saved to {}", filename);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_value() {
        assert_eq!(transform_value(-10).unwrap(), 0); // Negative values capped at 0
        assert_eq!(transform_value(50).unwrap(), 50); // Valid value unchanged
        assert_eq!(transform_value(150).unwrap(), 100); // Values over 100 capped at 100
    }

    #[test]
    fn test_etl_process() {
        let raw = vec![
            RawData { id: 1, value: -20 },
            RawData { id: 2, value: 120 },
            RawData { id: 3, value: 80 },
        ];
        let cleaned = extract_transform_load(raw).unwrap();

        assert_eq!(cleaned.len(), 3);
        assert_eq!(cleaned[0].value, 0); // Negative value transformed
        assert_eq!(cleaned[1].value, 100); // Outlier capped
        assert_eq!(cleaned[2].value, 80); // Valid value unchanged

        // Verify total and average
        let total: i32 = cleaned.iter().map(|data| data.value).sum();
        let average: f32 = total as f32 / cleaned.len() as f32;

        assert_eq!(total, 180);
        assert!((average - 60.0).abs() < f32::EPSILON); // Allowing for floating-point comparison
    }
}