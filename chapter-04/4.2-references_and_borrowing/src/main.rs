fn main() {
    // Rules of References
    // 1. At any given time, you can have *either* but not both of:
    //  - One mutable reference
    //  - Any number of immutable reference
    // 2. References must always be valid

    // Immutable References
    let mut x = String::from("Hello");
    let len = calculate_length(&x); // Create a reference that refers to the value of x but not own it.
    // change_immutable(&x);
    println!("The length of \"{}\" is {}.", x, len); // The function above did not take the ownership.

    // Mutable References
    {
        change(&mut x);
        // let r1 = &mut x; // Will result in an error, mutable reference can only have one at a time.
    } // The original mutable reference goes out of scope here, so now we can make a new one with no problem.
    // let r2 = &mut x; // Will result in an error, see the next line.
    // println!("x = \"{}\", r2 = \"{}\".", x, r2); // Because cannot combine mutable and immutable references
    println!("x = \"{}\".", x);
    // x in the upper line seems to be a immutable reference
    
    // Dangling Reference
    // let reference_to_nothing = dangle(); // Will result in a compile time error
    let not_a_reference = no_dangle();
    println!("{}", not_a_reference);
}

fn calculate_length(s: &String) -> usize { // Use &String instead of String.
    return s.len(); // &String s here is pointing at String x.
} // s goes out of scope, but because it does not have ownership of x so nothing happens.

// fn change_immutable(some_string: &String) {
//     some_string.push_str(", world!"); // Will result in an error.
//     Because the reference here is immutable.
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!"); // It will work
}

// fn dangle() -> &String { // Returnes a reference to a String
//     let s = String::from("Hello"); // New string s.
//     return &s; // Return a reference.
// } // s goes out of scope and being dropped.

fn no_dangle() -> String {
    let s = String::from("Hello");
    return s; // The solution is to return a string directly.
}
