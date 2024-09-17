/*
This code uses a HashMap to count and store the frequency of each number in A vector.
 */

use std::collections::HashMap;

fn main() {
    let fruits = vec![
        "banana", "cherry", "cherry", "apple", "banana", "apple", "apple", "apple", "cherry",
    ];

    let mut fruit_counts = HashMap::new();

    // Count the frequency of each fruit and add the HashMap
    for &fruit in &fruits {
        // return mut ref to the fruit value, if fruit not there add and enter count as 0
        let count = fruit_counts.entry(fruit).or_insert(0);
        *count += 1; // dereference count to get the value and increment it
    }

    // Print the frequency of each fruit
    for (fruit, count) in &fruit_counts {
        println!("{}: {}", fruit, count);
    }

    // To print the result sorted by frequency of counts
    //  we must converted to a vector of tuples and then
    // sort by the second element of the tuple using the 1. sort_by 2. sort by function
    // Collect (fruit, count) pairs into a vector of tuples
    let mut fruit_count_vec: Vec<_> = fruit_counts.into_iter().collect();

    // Here are two patterns to sort the vector by count of fruits in  
    // descending order
    // _ is used to ignore the fruit str.
    // 1. sort_by function
    // fruit_count_vec.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));

    // Alternatively we can use sort_by_key and the std::cmp::Reverse transformation func
    fruit_count_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    // Print the fruits and their counts in sorted order
    println!("\nOrdered by count of the fruit:");
    for (fruit, count) in &fruit_count_vec {
        println!("{}: {}", fruit, count);
    }
}
