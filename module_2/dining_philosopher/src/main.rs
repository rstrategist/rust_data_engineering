/*
* The dining philosophers problem involves multiple threads needing
* synchronised access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

impl fmt::Debug for Fork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fork {{ id: {} }}", self.id)
    }
}

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    // Philosopher Constructor
    fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    // Acquire lower numbered fork first to prevent deadlock
    fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        // Each fork is locked (picked-up) in turn
        // After eating, the philosopher puts down the forks.
        // The locks are automatically released when the _first_guard
        // and _second_guard go out of scope at the end of the function.
        let _first_guard = first_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _second_guard = second_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1)); // Eats for 1 second
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    // Here, we create 4 forks, each represented by an `Arc<Fork>`. Let's break this down:
    // - `Arc::new()` is used to create a thread-safe reference-counted pointer to each `Fork`.
    // - Each `Fork` contains an `id` and a `Mutex<()>`.
    // - The `Mutex` is used to ensure exclusive access to the fork. The `()` inside `Mutex<()>`
    // is just a unit type, as we don't need to store any data in the mutex itself - we're just
    // using it for synchronization.
    // - `Arc` (Atomic Reference Counting) allows multiple ownership of the same data across threads.
    // It's necessary here because multiple philosophers (threads) will need to access the same forks.

    let forks = (0..4)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    // Print the forks
    println!("Forks:");
    for fork in &forks {
        println!("{:?}", fork);
    }

    // Create 15 philosophers
    // For each philosopher:
    // - Arc::clone() is used to create new reference-counted pointers to the left and right
    // forks. This increases the reference count but doesn't create new forks.
    // - Each philosopher gets its own id, name, and references to its left and right forks.
    let philosophers = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (name, left, right))| {
        Philosopher::new(
            id as u32,
            name,
            Arc::clone(&forks[left]),
            Arc::clone(&forks[right]),
        )
    })
    .collect::<Vec<_>>();

    // Record start time of the simulation
    let start = Instant::now();

    // Spawn a thread for each philosopher
    // This section creates handles by:
    // - Iterating over the philosophers.
    // - For each philosopher, it spawns a new thread that calls the `eat()` method.
    // - The `move` keyword is used to move ownership of the philosopher into the thread.
    // - Each thread gets its own copy of the `Arc` pointers to the forks, allowing safe
    // concurrent access.
    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    //  Wait for all threads to finish (all philosophers finish eating)
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}

/*
Throughout this process, Mutex and Arc work together to provide safe concurrent access to
shared resources:

Arc allows multiple threads to have ownership of the same data (forks in this case).
Mutex ensures that only one thread can access a fork at a time.
When a philosopher tries to pick up a fork, they attempt to lock the Mutex.
If the lock is acquired, they have exclusive access to the fork.
If not, they wait until the fork becomes available.
This prevents data races and ensures that no two philosophers can use the same fork
simultaneously.

The use of Arc allows the forks to be shared between threads without knowing in advance
how long they need to live or which thread will be the last to use them. When all Arc
pointers to a fork are dropped (i.e., when the program ends), the fork is automatically
deallocated.
 */
