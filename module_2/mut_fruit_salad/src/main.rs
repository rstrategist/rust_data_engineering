// Creates a vector of strings in fruit_salad and use .push .pop .sort
// and custom functions to manipulate the string vector.
// Two different methods to remove elements from the vector of
// differing time complexity.

fn remove_fruit(fruit_salad: &mut Vec<String>, fruit_to_remove: &str) -> bool {
    // Removes only the first matching element
    // Get the index of the fruit to be removed
    if let Some(index) = fruit_salad
        .iter()
        .position(|fruit| fruit == fruit_to_remove)
    {
        // Remove the fruit
        fruit_salad.remove(index);
        true // return true if fruit found and removed
    } else {
        false
    }
}

// Function to remove all occurrences of a specific fruit from the fruit salad using the retain method
fn remove_all_fruits_retain(fruit_salad: &mut Vec<String>, fruit_to_remove: &str) -> usize {
    // Store the original length of the vector
    let original_length = fruit_salad.len();
    // Use the retain method to remove all occurrences of the fruit
    fruit_salad.retain(|fruit| fruit != fruit_to_remove);
    // Return the number of fruit removed
    original_length - fruit_salad.len()
}

// Function to remove all occurrences of a specific fruit from the fruit salad using a manual approach
fn remove_all_fruits_manual(fruit_salad: &mut Vec<String>, fruit_to_remove: &str) -> usize {
    let mut count = 0;
    // Continue removing fruits while there are matches. Time complexity at worst O(n^2)
    while let Some(index) = fruit_salad
        .iter()
        .position(|fruit| fruit == fruit_to_remove)
    {
        // Remove the fruit at the found index
        fruit_salad.remove(index);
        // Increment the count of removed fruits
        count += 1;
    }
    // Return the number of fruits removed
    count
}

fn main() {
    let mut fruit_salad = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "pear".to_string(),
        "papaya".to_string(),
        "kiwi".to_string(),
        "rasberries".to_string(),
    ];

    println!("\nOriginal fruit salad: {:?}", fruit_salad);

    fruit_salad.push("grapes".to_string());
    println!("\nFruit salad contains: {:?}", fruit_salad);

    fruit_salad.push("mango".to_string());
    fruit_salad.push("mango".to_string());
    fruit_salad.push("mango".to_string());
    println!("\nFruit salad contains: {:?}", fruit_salad);

    println!("\nPopped-off {:?}", fruit_salad.pop().unwrap());

    // Sort fruit salad vector
    fruit_salad.sort();
    println!("\nSorted fruit salad now contains: {:?}", fruit_salad);

    // Remove a fruit
    let fruit_to_remove = "mango";
    remove_fruit(&mut fruit_salad, fruit_to_remove);

    let removed = remove_fruit(&mut fruit_salad, fruit_to_remove);

    if removed {
        println!("\nRemoved '{}' from the fruit salad", fruit_to_remove);
    } else {
        println!("'{}' was not found in the fruit salad", fruit_to_remove);
    }

    println!("Updated fruit salad: {:?}", fruit_salad);

    // Removing all of a specific type of fruit using two different methods
    let mut fruit_salad1 = fruit_salad.clone();

    // Slow method of time complexity: O(n^2) in the worst case
    let removed_count = remove_all_fruits_manual(&mut fruit_salad, fruit_to_remove);
    println!(
        "\nUsing manual removal: Removed {} '{}' from the fruit salad",
        removed_count, fruit_to_remove
    );
    println!("Updated fruit salad (manual): {:?}", fruit_salad);

    // Faster method of time complexity: O(n), where n is the number of elements in the vector
    let removed_count1 = remove_all_fruits_retain(&mut fruit_salad1, fruit_to_remove);
    println!(
        "\nUsing retain: Removed {} '{}' from the fruit salad",
        removed_count1, fruit_to_remove
    );
    println!("Updated fruit salad (retain): {:?}", fruit_salad1);
}
