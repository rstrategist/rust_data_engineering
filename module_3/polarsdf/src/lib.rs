// utilities for working with polars dataframes
//
use polars::prelude::*;
use std::fs::File;

//read in a csv file
pub fn read_csv(path: &str) -> DataFrame {
    let file = File::open(path).expect("Could not open file");
    CsvReader::new(file).finish().expect("Failed to read CSV")
    //CsvReader::new(file).finish().unwrap()
    //CsvReader::from_path(path).unwrap().finish().unwrap()
}

//print "n" rows of a dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

//print the schema of a dataframe
pub fn print_schema(df: &DataFrame) {
    println!("{:?}", df.schema());
}

//print the shape of a dataframe
pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}
