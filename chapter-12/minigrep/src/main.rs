use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Reading the Argument Values
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for \"{}\" in file \"{}\".", query, filename);

        let mut f = File::open(filename).expect("File not found.");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Error while reading the file.");
        println!("The text here is:\n{}", contents);
    } else if args.len() == 1 {
        println!("Usage: bla bla");
    } else {
        println!("Invalid arguments.");
    }
}
