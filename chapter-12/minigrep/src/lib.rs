use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 3 {
            return Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            })
        } else if args.len() == 1 {
            return Err("Usage: <query> <filename>");
        } else {
            return Err("Invalid request");
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Searching for \"{}\" in file \"{}\".", config.query, config.filename);
    let mut f = File::open(config.filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error while reading the file.");
    println!("The text here is:\n{}", contents);
    return Ok(());
}
