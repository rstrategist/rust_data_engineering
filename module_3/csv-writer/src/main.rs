// Example code for reading data from a csv, updating values and writing to a new csv

// Rust CSV Cookbook: https://docs.rs/csv/1.0.0/csv/cookbook/index.html
// Cookbook Github (see examples folder): https://github.com/BurntSushi/rust-csv

// 1. Reading the CSV File: We open products.csv in read mode and create a Reader to parse the CSV file.
// 2. Processing Each Record: We iterate through each record, extract the product name and price, calculate a 10% discount, and store the results in a vector.
// 3. Writing to a New CSV File: We open discounted_products.csv in write mode, write the header row, and then write each product’s name and discounted price to the new file.
// 4. Flushing the Writer: Finally, we flush the writer to ensure all data is written to the file.

use csv::{Reader, Writer};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // Import the csv crate and open products.csv in read mode using csv::Reader
    let file = File::open("data/products.csv")?;
    let mut rdr = Reader::from_reader(file);

    // Create a vector to store the discounted products
    let mut discounted_products = Vec::new();

    // Iterate through the rows and extract the product name and price
    for result in rdr.records() {
        let record = result?;
        let product_name = record[0].to_string();
        let price: f64 = record[1].parse()?;

        // Calculate a 10% discount on the price
        let discounted_price = price * 0.9;

        // Store the product name and discounted price
        discounted_products.push((product_name.to_string(), discounted_price));
    }

    // Open discounted_products.csv in write mode using csv::Writer
    let mut wtr = Writer::from_path("data/discounted_products.csv")?;

    //Write the header row with column names
    wtr.write_record(["Fruit", "Price"])?;

    // Write each product's name and discounted price to the output file
    for (product_name, discounted_price) in discounted_products {
        wtr.write_record(&[product_name, discounted_price.to_string()])?;
    }

    // Open the CSV file in write mode
    let mut wtr = Writer::from_path("data/discounted_products.csv")?;

    // Flush the writer when finished to save the data
    wtr.flush()?; // Ensure data is written

    Ok(())
}
