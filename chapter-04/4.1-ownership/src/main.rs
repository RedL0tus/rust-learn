fn main() {
    variable_scope();
    string_type();
    memory_and_allocation();
    ownership_and_functions();
    return_values_and_scope();
    return_multiple_values();
}


fn variable_scope() {
    // Variable Scope

    // A variable is not valid before it's been declared.
    // When a variable comes into scope, it's valid.
    // It remains so until it goes out of scope.
    let x = "Hello, world!"; // From now on, x is valid.
    // Do stuff with s
    println!("{}",x);
} // This scope is over, s is no longer valid.

fn string_type() {
    let mut x = String::from("Hello"); // Declear a mutable string.
    x.push_str(", world!"); // Appends a literal to a String.
    println!("{}", x); // This will print "Hello, world!"
    // String can be mutated but literals cannot.
} // Rust will automatically drop unnecessary variables.

fn memory_and_allocation() {
    // Ways Variables and Data Interact

    // Move
    let x = String::from("Hello"); // x is valid from this point forward
    let y = x; // y is valid from now on, but x is no longer valid.
                // in order to prevent double free error.
                // the pointer of x had moved to y.
    // println!("{}, world!", x); // Will cause a error.
    println!("{}, world!", y);

    // Clone
    let x = String::from("Hello"); // x is valid from now on.
    let y = x.clone(); // y is valid from now on, and x keeps valid.
    println!("{}, world!", x);
    println!("{}, world!", y); // Both will work, because the heap got copied.

    // Stack-Only Data: Copy
    let x = 5;
    let y = x; // x will remain valid
    println!("x = {}, y = {}", x, y); // Will work.
    // If a type has the 'Copy' trait, an older variable is still usable after assignment.
    // All integer, floating point, boolean, character types and the tuples that only
    // contains types that are also Copy are 'Copy'.
    // Rust won't let us annotate a type with 'Copy' trait if the type or any of its parts,
    // has implemented the 'Drop' trait. 
}

fn ownership_and_functions() {
    let x = String::from("Hello"); // x comes into scope.
    takes_ownership(x); // x's value moves into the function, it's no longer valid here.
    // println!("{}", x); // Will result in a compile time error.
    
    let x = 5; // x comes into scope.
    makes_copy(x); // x would move in to the function, but i32 is copy.
    println!("x = {}", x); // So it's OK to still use x afterward.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
    // Do stuff with some_string.
} // some_string goes out of scope and 'drop' is called, memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
    // Do stuff with some_integer
} // some_integer goes out of scope but nothing special happens.

fn return_values_and_scope() {
    let x = gives_ownership(); // gives_ownership moves its return falue into x.
    let y = String::from("Hello"); // y comes into scope.
    let z = takes_and_gives_back(y); // y is moved into takes_and_gives_back
                                    // and it return value into z.
    println!("x = {}", x);
    // println!("y = {}", y); // y was moved so it will result in a compile time error.
    println!("z = {}", z);
} // x and z goes out of scope and dropped, y goes out of scope but it was moved.

fn gives_ownership() -> String { // This function will move its return value into the calling function.
    let some_string = String::from("Hello"); // some_string comes into scope.
    return some_string; // some_string is returned and moves out to the callingg function
}

// takes_and_gives_back will take a string and return one.
fn takes_and_gives_back(some_string: String) -> String { // some_string comes into scope.
    // Do stuff with some_string.
    return some_string; // some_string is returned and moves out to the calling function.
}

fn return_multiple_values() {
    let x = String::from("Hello");
    let (y, len) = calculate_length(x);
    println!("The length of '{}' is {}.", y, len);
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len(); // returns the length of a sting.
    return (some_string, length);
}
