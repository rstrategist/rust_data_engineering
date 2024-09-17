/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
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

    // Remove a fruit from the front and back and print
    println!("Removing one fruit from the front and back of the vecdeque:");
    fruit.pop_front();
    fruit.pop_back();
    println!("After removing a fruit from the front and back:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // converted back to vector to choose a random fruit
    let fruit: Vec<_> = fruit.into_iter().collect();

    // Select a random fruit using choose method
    match fruit.choose(&mut rng) {
        Some(random_fruit) => {
            println!("\nRandom fruit picked from the salad: {}", random_fruit)
        }
        None => println!("\nCouldn't pick a random fruit. This shouldn't happen!"),
    }

    // A function to pick 3 random fruits
    fn pick_random_fruits<'a>(fruit: &[&'a str], num: usize) -> Vec<&'a str> {
        let mut rng = thread_rng();
        let mut random_fruits: Vec<_> = fruit.iter().cloned().collect();
        random_fruits.shuffle(&mut rng);
        random_fruits.into_iter().take(num).collect()
    }

    // Pick 3  random fruits from the salad
    let num_fruits = 3;
    let random_fruits = pick_random_fruits(&fruit, num_fruits);
    println!(
        "\n{} random fruits picked from the salad: {:?}",
        num_fruits, random_fruits
    );
}

// for (i, fruit) in fruits.iter().enumerate() {
//     println!("{}. {}", i + 1, fruit);
// }