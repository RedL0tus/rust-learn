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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INCENSITIVE").is_err();
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines()
                   .filter(|line| line.contains(&query))
                   .collect();
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return contents.lines()
                   .filter(|line| line.to_lowercase().contains(&query))
                   .collect();
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
