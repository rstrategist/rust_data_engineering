/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");
    fruit.push_front("Apple");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Shuffle the fruit. Need to convert LinkedList to a Vec to use the shuffle method.
    let mut rng = thread_rng();
    let mut shuffled_fruit: Vec<_> = fruit.into_iter().collect();
    shuffled_fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    fruit = shuffled_fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Remove a fruit at a specific index (e.g., index 2)
    let index_to_remove = 2;
    let removed_fruit = remove_at(&mut fruit, index_to_remove).unwrap_or("No fruit removed");

    // Print the removed fruit
    println!(
        "Removed fruit at index {}: {}",
        index_to_remove, removed_fruit
    );

    // // Remove a fruit from any position in the LinkedList
    // // Remove a fruit at a specific index (e.g., index 2). NOTE: .remove() method marked
    // // as unstable by the compiler.
    // let index_to_remove = 2;
    // let removed_fruit = fruit.remove(index_to_remove).unwrap_or("No fruit removed");

    // // Print the removed fruit
    // println!(
    //     "Removed fruit at index {}: {}",
    //     index_to_remove, removed_fruit
    // );

    // Print out the updated fruit salad
    println!("Updated Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

// Function to remove element at index from LinkedList
// We have done this since the .remove() method of LinkedList is marked as unstable
fn remove_at<T>(list: &mut LinkedList<T>, index: usize) -> Option<T> {
    // Check if index is out of bounds
    if index >= list.len() {
        return None;
    }

    // Iterate through the list to find the element at the specified index
    let mut current_index = 0;
    let mut result = None;

    for _ in 0..=index {
        if let Some(item) = list.pop_front() {
            if current_index == index {
                result = Some(item);
            } else {
                list.push_back(item);
            }
            current_index += 1;
        }
    }

    result
}
