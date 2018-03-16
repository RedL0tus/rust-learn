fn main() {
    let mut s = String::new(); // Define a new string.
    let s1 = "b".to_string();
    let tic = "tic".to_string();
    let tac = String::from("tac");
    let toe = String::from("toe");
    let hello = String::from("こんにちは"); // Can store different languages.
    let s2 = String::from("a");
    let s3 = s1 + &s2; // s1 will be removed but s2 will not.

    // Updating a String
    s.push_str("foo");
    s.push_str(&s3);
    s.push('r'); // Add a single character to a String.

    // Output
    println!("{}", s);
    println!("{}", hello);
    println!("{}", tic + "-" + &tac + "-" + &toe);

    // Rust string don't support indexing.
    // let s = String::from("Hello");
    // let h = s[0]; // Will rusult in an error.
    
    // Internal Representation
    println!("The length of String s is: {}.", s.len());

    // Slicing Strings
    println!("{}", &hello[0..3]);

    // Iterating over Strings
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}
