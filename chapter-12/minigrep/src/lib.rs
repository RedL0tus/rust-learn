use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 3 {
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            return Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive,
            });
        } else if args.len() == 1 {
            return Err("Usage: <query> <filename>");
        } else {
            return Err("Invalid request");
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut results = Vec::new();
    if config.case_sensitive {
        results = search(&config.query, &contents);
    } else {
        results = search_case_insensitive(&config.query, &contents);
    };
    for line in results {
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Salted";
        let contents = "\
Poisonous Salted Fish
Can't be used as food.
100 BTC per serve, only at fishmart!";
        assert_eq!(
            search(query, contents),
            vec!["Poisonous Salted Fish"]
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "sAlTeD";
        let contents = "\
Poisonous Salted Fish
Can't be used as food.
100 BTC per serve, only at fishmart!";
        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Poisonous Salted Fish"]
        );
    }
}
