use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    // The API of `Mutex<T>`
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6; // `Mutex<T>` provides interior mutability
        }
        println!("m = {:?}", m);
    }
    // Sharing a `Mutex<T>` between multiple threads
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..10 {
            println!(">>> Thread {} has started.", i);
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
            println!(">>> Thread {} is going to end.", i);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
