use std::collections::VecDeque;

fn main() {
    // Create a new VecDeque to represent the queue
    let mut queue: VecDeque<&str> = VecDeque::new();

    // Simulate people joining the queue
    queue.push_back("Alice");
    queue.push_back("Bob");
    queue.push_back("Charlie");

    println!("Queue after people join: {:?}", queue);

    // Simulate people leaving the queue
    if let Some(person) = queue.pop_front() {
        println!("{} has left the queue.", person);
    }

    println!("Queue after one person leaves: {:?}", queue);

    // Add another person to the queue
    queue.push_back("Dave");

    println!("Queue after Dave joins: {:?}", queue);

    // Continue processing the queue
    while let Some(person) = queue.pop_front() {
        println!("{} has left the queue.", person);
    }

    println!("Queue after processing all: {:?}", queue);
}
