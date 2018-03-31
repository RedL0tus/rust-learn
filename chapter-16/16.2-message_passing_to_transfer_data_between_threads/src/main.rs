use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    println!(">>> Main() started.");

    let (tx1, rx) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx1); // Create multiple producers by cloning the transmitter

    let handle1 = thread::spawn(move || {
        println!(">>> Spawned thread 1 started.");
        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // The `send` function takes ownership of its parameter
            thread::sleep(Duration::from_millis(150));
        }
        // println!("Vals is: {:?}", vals); // Will fail
        println!(">>> Spawned thread 1 is going to end.");
    });

    let handle2 = thread::spawn(move || {
        println!(">>> Spanwed thread 2 started.");
        let vals = vec![
            String::from("More"),
            String::from("Messages"),
            String::from("For"),
            String::from("You"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        println!(">>> Spawned thread 2 is going to end.");
    });

    for received in rx {
        println!(">>> Main thread got: {}", received);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!(">>> Main() is going to end.");
}
