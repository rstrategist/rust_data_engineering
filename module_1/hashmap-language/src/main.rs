use std::collections::HashMap;

// Function to create and return the initialized HashMap
fn create_languages_map() -> HashMap<String, i32> {
    let mut languages = HashMap::new();

    // Insert key-value pairs (same as before)
    languages.insert(String::from("C"), 1972);
    languages.insert(String::from("C++"), 1983);
    languages.insert(String::from("Java"), 1995);
    languages.insert(String::from("Python"), 1991);
    languages.insert(String::from("Rust"), 2010);
    languages.insert(String::from("JavaScript"), 1995);
    languages.insert(String::from("Swift"), 2014);
    languages.insert(String::from("Ruby"), 1995);
    languages.insert(String::from("Go"), 2009);
    languages.insert(String::from("Kotlin"), 2011);
    languages.insert(String::from("TypeScript"), 2012);
    languages.insert(String::from("Perl"), 1987);
    languages.insert(String::from("Haskell"), 1990);
    languages.insert(String::from("Scala"), 2003);
    languages.insert(String::from("PHP"), 1995);
    languages.insert(String::from("Assembly"), 1950);
    languages.insert(String::from("Erlang"), 1986);
    languages.insert(String::from("Julia"), 2012);
    languages.insert(String::from("R"), 1993);
    languages.insert(String::from("Matlab"), 1984);
    languages.insert(String::from("Dart"), 2011);
    languages.insert(String::from("Groovy"), 2003);
    languages.insert(String::from("C#"), 2000);

    languages
}

// Function to calculate and return weights for each language
fn calculate_weights(languages: &HashMap<String, i32>) -> HashMap<String, f64> {
    let mut weights = HashMap::new();
    let current_year = 2024;

    // Find the maximum and minimum years
    let max_year = *languages.values().max().unwrap_or(&current_year);
    let min_year = *languages.values().min().unwrap_or(&current_year);

    // Calculate weights
    for (language, &year) in languages.iter() {
        let _age = current_year - year;
        // Normalize to 0.0-1.0 range
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) + 1.0; // Scale to 1.0-100.0 range
        weights.insert(language.clone(), weight);
    }

    weights
}

fn main() {
    // Call the function to get the HashMap of languages
    let languages = create_languages_map();

    // Call the function to calculate weights based on language age
    let weights = calculate_weights(&languages);

    // Convert HashMap to vector of tuples for sorting
    let mut sorted_weights: Vec<(&String, &f64)> = weights.iter().collect();
    // Sort by weight in descending order (highest weight first)
    sorted_weights.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Print the sorted weights
    println!("Languages by normalized weight (highest to lowest):");
    for &(language, weight) in &sorted_weights {
        println!("{}: {}", language, *weight as i32); // Print weight as integer
    }
}
