use std::{thread, time};

fn main() {
    // if expressions
    let condition: bool = true;
    let x = if condition { // Using if in a let statement
        5
    } else {
        6 // Types must be matched
        // "six" // will not work
    };
    println!("The value of x is: {}", x);
    let mut x = 3;
    if x % 4 == 0 { // Must be a boolean expression
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }

    // Repetition with Loops

    // Conditional Loops with while
    while x != 0 {
        println!("The value of x is: {}", x);
        x = x - 1;
    }

    let x = [10, 20, 30, 40, 50];
    for i in x.iter() {
        println!("The value of x's element is: {}", i);
    }

    // Repeating Code with loop
    loop {
        println!("怒った？");
        thread::sleep(time::Duration::from_millis(100));
    }
}
