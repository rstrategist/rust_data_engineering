use std::collections::HashMap;

fn main() {
    let fruits = vec![
        "apple", "banana", "cherry", "apple", "banana", "apple", "apple", "apple", "cherry",
    ];

    let mut fruit_counts = HashMap::new();

    // Count the frequency of each fruit
    for &fruit in &fruits {
        // return mut ref to the fruit value, if fruit not there add and enter count as 0
        let count = fruit_counts.entry(fruit).or_insert(0);
        *count += 1; // dereference count to get the value and increment it
    }

    // Print the frequency of each fruit
    for (fruit, count) in &fruit_counts {
        println!("{}: {}", fruit, count);
    }
}
