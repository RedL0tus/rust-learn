use std::thread;
use std::time::Duration;

fn say_hi(x: i32, thread_name: &str) {
    for i in 1..x+1 {
        println!("Hi, this is {} from {}!",i , &thread_name);
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    {
        // Creating a new thread with `spawn`
        let handle = thread::spawn(|| {
            say_hi(10, "spawned thread");
        });
        // The following line will let the block the main thread until the spawned
        // thread finishes.
        // handle.join().unwrap();

        say_hi(5, "main thread");

        // Waiting for all threads to finish using `join` handles
        handle.join().unwrap();
    }
    // Using `move` closures with threads
    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // drop(v); // Oh, no!

        handle.join().unwrap();
    }
}
