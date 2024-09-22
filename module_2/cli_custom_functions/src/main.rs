/*
Usage:

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"
or
cargo run -- --fruits "apple, pear" --output write_fruit_salad.csv

 */

use clap::Parser;
use lib_functions::create_fruit_salad;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Rashid Rasul <rashid.rasul@me.com>",
    about = "CLI Custom Functions"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
    /// Output CSV file
    #[clap(short, long)]
    output: Option<String>,
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}

fn display_fruit_salad(fruits: &Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn write_fruit_salad_to_csv(fruits: &Vec<String>, filename: &str) {
    let path = std::env::current_dir()
        .expect("Could not get current directory")
        .join(filename);
    println!("Full path of the file: {}", path.display());
    let mut file = File::create(&path).expect("Could not create file");
    let csv_content = fruits.join(",");
    file.write_all(csv_content.as_bytes())
        .expect("Could not write to file");
}

fn main() {
    let opts: Opts = Opts::parse();

    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Could not read file");
            csv_to_vec(&fruits)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    // Creates a fruit salad and shuffles it
    let mut fruit_salad = create_fruit_salad(fruit_list);
    // Display fruit salad
    display_fruit_salad(&fruit_salad);

    // Write fruit salad to CSV file if output is specified
    // if let Some(output_file) = opts.output {
    //     write_fruit_salad_to_csv(&fruit_salad, &output_file);
    //     println!("Fruit salad written to {}", output_file);
    // }

    // Write fruit salad to CSV file if output is specified
    if let Some(output_file) = opts.output {
        let root_dir = std::env::current_dir().expect("Could not get current directory");
        let output_path = root_dir.join(output_file);
        write_fruit_salad_to_csv(&fruit_salad, output_path.to_str().unwrap());
        println!("Fruit salad written to {}", output_path.display());
    }
}
