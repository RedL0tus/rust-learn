extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// To panic or not to panic
// It's OK to panic in examples or prototype codes.
// And it's safe to use unwrap on things you are sure that it won't produce any error.
// Compiler isn't smart enought to detect it.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() { // Use i32 to handle potentially negative number
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        if guess < 1 || guess > 100 { // Handle invalid number
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
