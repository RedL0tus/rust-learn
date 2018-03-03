fn main() {
    println!("Hello, world!");
    another_function();
    yet_another_function(1, 2);
    statements_and_expressions();
    println!("The sum of 1 and 2 is: {}", sum(1, 2));
}

fn another_function() {
    println!("Another function");
}

fn yet_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statements_and_expressions() {
    let x = 6; // It will work
    println!("The value of x is: {}", x);
    //let x = (let y = 6); // It won't work
    let y = {
        let x = 3;
        x + 1 // No semicolon here
    }; // It will work
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y; // Semicolon isn't necessary in the last line.
}
