use csv::{Writer, Reader};
use std::{error::Error, fs::File, io, process};

fn main() -> Result<(), Box<dyn Error>> {
    // Import the csv crate and open products.csv in read mode using csv::Reader
    let file = File::open("data/products.csv")
    let mut rdr = Reader::from_reader(io::stdin());
    


    // Iterate through the rows and extract the product name and price

    // Calculate a 10% discount on the price

    // Open discounted_products.csv in write mode using csv::Writer

    //Write the header row with column names

    // Write each product's name and discounted price to the output file

    // Flush the writer when finished to save the data

    // Open the CSV file in write mode
    let mut wtr = Writer::from_path("data/output.csv")?;

    // Write the header row
    wtr.write_record(["Fruit", "Price"])?;

    // Write each fruit and its price to the CSV file
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?; // Convert price to string
    }

    wtr.flush()?; // Ensure data is written

    Ok(())
}
