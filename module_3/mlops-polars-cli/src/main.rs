// Import necessary libraries and modules
use clap::{App, Arg, SubCommand};
use log::{info, warn};
use polars::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    // Initialize logging
    env_logger::init();

    // Define CLI structure using Clap
    let matches = App::new("Polars DataFrame CLI Tool")
        .version("0.1.0")
        .author("Rashid Rasul rashid.rasul@me.com>")
        .about("A CLI tool for analysing and transforming datasets with Polars")
        .subcommand(
            SubCommand::with_name("schema")
                .about("Displays the schema of the dataset")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .help("Path to the CSV file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("shape")
                .about("Displays the shape of the dataset")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .help("Path to the CSV file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("sort")
                .about("Sorts the dataset by a column")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .help("Path to the CSV file"),
                )
                .arg(
                    Arg::with_name("column")
                        .required(true)
                        .help("Column to sort by"),
                )
                .arg(
                    Arg::with_name("rows")
                        .required(false)
                        .help("Number of rows to display"),
                )
                .arg(
                    Arg::with_name("order")
                        .required(false)
                        .help("Sort order: 0 for ascending, 1 for descending"),
                ),
        )
        .subcommand(
            SubCommand::with_name("filter")
                .about("Filters the dataset by a column condition")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .help("Path to the CSV file"),
                )
                .arg(
                    Arg::with_name("column")
                        .required(true)
                        .help("Column to filter on"),
                )
                .arg(
                    Arg::with_name("condition")
                        .required(true)
                        .help("Condition to apply (e.g., eq, gt, lt)"),
                )
                .arg(
                    Arg::with_name("value")
                        .required(true)
                        .help("Value to filter by"),
                ),
        )
        .get_matches();

    // Match subcommands and execute corresponding logic
    if let Some(matches) = matches.subcommand_matches("schema") {
        if let Some(file) = matches.value_of("file") {
            display_schema(file);
        }
    } else if let Some(matches) = matches.subcommand_matches("shape") {
        if let Some(file) = matches.value_of("file") {
            display_shape(file);
        }
    } else if let Some(matches) = matches.subcommand_matches("sort") {
        if let Some(file) = matches.value_of("file") {
            let column = matches.value_of("column").unwrap();
            let rows = matches
                .value_of("rows")
                .unwrap_or("5")
                .parse::<usize>()
                .unwrap();
            let order = matches
                .value_of("order")
                .unwrap_or("0")
                .parse::<usize>()
                .unwrap();
            sort_dataset(file, column, rows, order);
        }
    } else if let Some(matches) = matches.subcommand_matches("filter") {
        if let Some(file) = matches.value_of("file") {
            let column = matches.value_of("column").unwrap();
            let condition = matches.value_of("condition").unwrap();
            let value = matches.value_of("value").unwrap();
            filter_dataset(file, column, condition, value);
        }
    }
}

/// Display the schema of the dataset
fn display_schema(file: &str) {
    let df = CsvReader::from_path(file).unwrap().finish().unwrap();
    println!("Schema:\n{:?}", df.schema());
}

/// Display the shape of the dataset
fn display_shape(file: &str) {
    let df = CsvReader::from_path(file).unwrap().finish().unwrap();
    println!("Shape: {:?}", df.shape());
}

/// Sort the dataset by a specific column
fn sort_dataset(file: &str, column: &str, rows: usize, order: usize) {
    let mut df = CsvReader::from_path(file).unwrap().finish().unwrap();
    let ascending = order == 0;
    df = df.sort(column, ascending).unwrap();
    println!("Sorted Data:\n{:?}", df.head(Some(rows)));
}

/// Filter the dataset by a specific condition
fn filter_dataset(file: &str, column: &str, condition: &str, value: &str) {
    let mut df = CsvReader::from_path(file).unwrap().finish().unwrap();
    let filtered = match condition {
        "eq" => df.filter(&df.column(column).unwrap().equal(value)).unwrap(),
        "gt" => df.filter(&df.column(column).unwrap().gt(value)).unwrap(),
        "lt" => df.filter(&df.column(column).unwrap().lt(value)).unwrap(),
        _ => panic!("Unsupported condition"),
    };
    println!("Filtered Data:\n{:?}", filtered);
}
