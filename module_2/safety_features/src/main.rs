/*
    This program demonstrates some of the security features of Rust.
    The following are implemented below:

    - Safe concurrency with threads and mutexes
    - Annotated references with lifetimes
    - Use of mutable and immutable bindings
    - Spawning of threads and passing data safely between them
    - Implemented a mutex to coordinate thread safety
    - Read from environment variables safely using ownership conventions
*/

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

/// Returns the longest of two string slices.
///
/// # Examples
///
/// ```
/// let string1 = "long string";
/// let string2 = "short";
/// let result = longest(string1, string2);
/// assert_eq!(result, "long string");
/// ```
///
/// # Arguments
///
/// * `x` - A string slice.
/// * `y` - Another string slice.
///
/// # Returns
///
/// The longest string slice.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Immutable binding
    let greeting = "Hello";
    // Mutable binding
    let mut name = String::from("World");

    // Modify the mutable binding
    name.push_str("!");

    println!("{}, {}", greeting, name);

    // Read from environment variables
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't interpret {}: {}", key, e),
    }

    // Shared data between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex and increment the counter
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Using the function with lifetime annotations
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
