// Computes the sum of squares of a vector sequentially and in parallel using Rayon.
// Adjust the vector size and number of threads to see how it impacts performance.
// In this simple case, the sequential version is likely to be faster due to the overhead of parallelisation.
// See notes at the end of the code for further discussion on this.

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::time::Instant;

fn main() {
    // Create a thread pool with a specified number of threads
    let num_threads = 4;
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    // Initialize data
    let data: Vec<i64> = (0..1000_000).collect();

    // Sequential computation
    let start = Instant::now();
    let sequential_sum: i64 = data.iter().map(|x| x * x).sum();
    let sequential_duration = start.elapsed();

    println!("Number of threads: {}", num_threads);
    println!("Sequential sum: {}", sequential_sum);
    println!("Sequential duration: {:?}", sequential_duration);

    // Parallel computation within the custom thread pool
    pool.install(|| {
        let start = Instant::now();
        let parallel_sum: i64 = data
            .par_iter() // parallel iterator
            .map(|x| x * x)
            .sum();
        let parallel_duration = start.elapsed();

        println!("Parallel sum: {}", parallel_sum);
        println!("Parallel duration: {:?}", parallel_duration);
    });
}

/*
1. Hyperparameters to Tweak for Performance
In parallel algorithms, one key hyperparameter to tweak is the number of threads.
Adjusting the number of threads can significantly impact performance.
For example, setting the number of threads to match the number of CPU cores can optimise resource utilisation.
However, setting it too high can lead to overhead from thread management, while setting it too low might not
fully utilise the available hardware.

2. Tradeoffs in Parallelising Algorithms
When parallelising algorithms, several tradeoffs exist:

Synchronisation Overhead: Managing access to shared resources can introduce delays.
Communication Costs: Data exchange between threads or processes can be expensive.
Load Balancing: Ensuring all threads have equal work can be challenging.
Scalability: Some algorithms do not scale well with increased parallelism due to inherent dependencies.

3. Scenarios Where Concurrency Does Not Improve Performance
Concurrency might not improve performance in scenarios where:

Overhead Exceeds Gains: The overhead of managing threads outweighs the benefits.
Memory Bandwidth: Limited memory bandwidth can become a bottleneck.
I/O Bound Tasks: Tasks that are limited by input/output operations rather than CPU.

4. Rust’s Mitigation of Race Conditions
Rust mitigates race conditions through its ownership model and type system. Key features include:

Ownership and Borrowing: Ensures that only one mutable reference or
multiple immutable references exist at a time.
Send and Sync Traits: Types that are safe to transfer across threads implement Send,
and types that are safe to share between threads implement Sync.

5. Impact of num_threads() Setting
The num_threads() setting in parallel libraries like Rayon controls the number of threads used
for parallel operations. Adjusting this setting can:

Optimise Performance: Matching the number of threads to the number of CPU cores can maximise performance.
Avoid Overhead: Too many threads can lead to context switching and synchronisation overhead.

6. Safety Guarantees with Rayon
Rayon provides several safety guarantees:

Data Race Prevention: Ensures that parallel iterators do not cause data races.
Safe Parallelism: Uses Rust’s type system to enforce safe parallel execution.
Automatic Load Balancing: Dynamically balances the workload across threads to optimise performance.
*/
