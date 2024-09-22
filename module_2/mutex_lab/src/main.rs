/*
Mutex that protects the data vector, and then we spawn three threads
that each acquire a lock on the mutex and modify an element of the vector.
The main thread then waits for all threads to finish before printing the
vector.

There are 3 implementations below.
1. Mutex with Arc.
2. Murtex with Condvar.
3. An unsafe case where we attempt to mutate a vector in
multiple threads without locking it => datarace. This will
not compile.
*/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/*
fn main() {
    // Need to wrap the Mutex in an Arc (Atomic Reference Counted).
    // Arc provides thread-safe reference counting and implements Clone,
    // allowing us to share the Mutex across multiple threads.
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    println!("Original data vector: {:?}", data.lock().unwrap());

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Data vector after addition via parallel threads: {:?}",
        data.lock().unwrap()
    );
}
*/

/*
Condvar (Conditional Variable) example showing how Condvar allows threads
to sleep and be woken up only when necessary, avoiding the inefficiency
of busy-waiting. Useful in data producer and consumer use cases,
a powerful mechanism for thread synchronisation.
- Only one thread can lock the Mutex at a time.
- The Mutex provides interior mutability, allowing safe mutation of the data
it protects.
- The Mutex lock is automatically released when it goes out of scope, ensuring
that other threads can acquire the lock.
*/

fn main() {
    let data = Arc::new((Mutex::new(vec![1, 2, 3]), Condvar::new()));
    let counter = Arc::new((Mutex::new(0), Condvar::new()));

    println!("Original data vector: {:?}", data.0.lock().unwrap());

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let (lock, _) = &*data;
                let mut data = lock.lock().unwrap();
                data[i] += 1;

                let (counter_lock, counter_cvar) = &*counter;
                let mut count = counter_lock.lock().unwrap();
                *count += 1;
                counter_cvar.notify_one();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let (counter_lock, counter_cvar) = &*counter;
    let mut count = counter_lock.lock().unwrap();
    while *count < 3 {
        count = counter_cvar.wait(count).unwrap();
    }

    let (lock, _) = &*data;
    let data = lock.lock().unwrap();
    println!(
        "Data vector after addition via parallel threads: {:?}",
        data
    );
}

/* Data race example
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
 */
