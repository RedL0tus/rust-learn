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
    let mut f = File::open(config.filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error while reading the file.");
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Salted";
        let contents = "\
Poisonous Salted Fish
Can't be used as food.
100 BTC per serve only at fishmart!";
        assert_eq!(
            search(query, contents),
            vec!["Poisonous Salted Fish"]
        );
    }
}
