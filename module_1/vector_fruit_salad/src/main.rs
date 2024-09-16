/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
We have updated the implementation to take the fruits as user input, shuffle and print.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io::{self, Write};

fn main() {
    let mut fruits: Vec<String> = Vec::new();
    let mut rng = thread_rng();

    println!("Welcome to the Fruit Salad Maker!");
    println!("Enter fruits to add to your salad. Type 'done' when finished.");

    loop {
        print!("Enter a fruit (or 'done' to finish): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "done" {
            break;
        }

        if !input.is_empty() {
            fruits.push(input.to_string());
            println!("Added {} to the salad!", input);
        }
    }

    if fruits.is_empty() {
        println!("No fruits were added. Your fruit salad is empty!");
    } else {
        println!("\nMixing the fruit salad...");
        fruits.shuffle(&mut rng);

        println!("Your fruit salad contains:");
        for (i, fruit) in fruits.iter().enumerate() {
            println!("{}. {}", i + 1, fruit);
        }

        // Select a random fruit using choose method
        match fruits.choose(&mut rng) {
            Some(random_fruit) => {
                println!("\nRandom fruit picked from the salad: {}", random_fruit)
            }
            None => println!("\nCouldn't pick a random fruit. This shouldn't happen!"),
        }
    }
}

// Initial version
// fn main() {
//     let mut fruit = vec![
//         "Orange",
//         "Fig",
//         "Pomegranate",
//         "Cherry",
//         "Apple",
//         "Pear",
//         "Peach",
//     ];

//     // Scramble (shuffle) the fruit
//     let mut rng = thread_rng();

//     fruit.shuffle(&mut rng);

//     // Print out the fruit salad
//     println!("Fruit Salad:");
//     for (i, item) in fruit.iter().enumerate() {
//         if i != fruit.len() - 1 {
//             print!("{}, ", item);
//         } else {
//             println!("{}", item);
//         }
//     }
// }
