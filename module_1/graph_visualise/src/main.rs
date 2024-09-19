use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate rasciigraph;

use rasciigraph::{plot, Config};

/// This program reads a list of cities and their distances from a source city (Lisbon)
/// from a file, and visualizes the distances as an ASCII graph in the terminal. The cities are printed
/// in order of increasing distance from the source city.
///
/// The distances are stored in a `HashMap` which is populated from a txt file, and the
/// cities are sorted based on their distances before printing and plotting.
///
/// The file format should look like this:
/// ```text
/// Lisbon:0.0
/// Madrid:502.56
/// Paris:1053.36
/// ...
/// ```

/// Reads a file and returns a hashmap of cities and distances from the source city.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path of the distances file.
///    "root/data/distances.txt"
///
/// # Returns
///
/// * A `HashMap` where the key is a `String` representing the city name,
///   and the value is an `f64` representing the distance from the source city.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use std::fs::File;
/// use std::io::{BufWriter, Write};
///
/// // Test setup: create a temporary file with sample data
/// let file_path = "test_distances.txt";
/// let sample_data = "Lisbon:0.0\nMadrid:502.56\nParis:1053.36\n";
/// let mut writer = BufWriter::new(File::create(file_path).unwrap());
/// writer.write_all(sample_data.as_bytes()).unwrap();
///
/// let distances = read_file(file_path).unwrap();
/// assert_eq!(distances["Lisbon"], 0.0);
/// assert_eq!(distances["Madrid"], 502.56);
/// assert_eq!(distances["Paris"], 1053.36);
///
/// std::fs::remove_file(file_path).unwrap(); // Cleanup after test
/// ```
///
fn read_file(file_path: &str) -> io::Result<HashMap<String, f64>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let mut distances = HashMap::new();

    for line in lines {
        if let Ok(entry) = line {
            let parts: Vec<&str> = entry.split(':').collect();
            if parts.len() == 2 {
                let city = parts[0].trim().to_string();
                if let Ok(value) = parts[1].trim().parse::<f64>() {
                    distances.insert(city, value);
                }
            }
        }
    }
    Ok(distances)
}

fn main() -> io::Result<()> {
    // Reading data from file
    let distances_map = read_file("data/distances.txt")?;

    // Extracting cities and distances and sorting by increasing distance
    let mut city_distance_pairs: Vec<(&String, &f64)> = distances_map.iter().collect();
    city_distance_pairs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    let cities: Vec<String> = city_distance_pairs
        .iter()
        .map(|&(city, _)| city.clone())
        .collect();
    let distances: Vec<f64> = city_distance_pairs
        .iter()
        .map(|&(_, &distance)| distance)
        .collect();

    // Print city path in order of increasing distance
    println!("{}", cities.join(" > "));

    // Plot ASCII graph
    println!(
        "{}",
        plot(
            distances.iter().copied().collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the read_file function by providing a sample file.
    #[test]
    fn test_read_file() {
        // Create a temporary file with sample data for the test
        let file_path = "data/test_distances.txt";
        let sample_data = "Lisbon:0.0\nMadrid:502.56\nParis:1053.36\n";

        std::fs::write(file_path, sample_data).unwrap();

        let distances = read_file(file_path).unwrap();

        // Validate contents
        assert_eq!(distances["Lisbon"], 0.0);
        assert_eq!(distances["Madrid"], 502.56);
        assert_eq!(distances["Paris"], 1053.36);

        // Cleanup after test
        std::fs::remove_file(file_path).unwrap();
    }
}
