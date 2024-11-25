use std::error::Error;

use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(arg) => arg,
        };

        let file_path = match args.next() {
            None => return Err("Didn't get a file path"),
            Some(arg) => arg,
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let query = query.as_str();

    search_case_sensitive(query, contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_ok() {
        let query = "Charon";
        let contents = "\
            No coins for Charon, unable to cross over worlds
            Dumped out in the river, found unworthy to pass now";
        assert_eq!(
            vec!["No coins for Charon, unable to cross over worlds"],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive_no_matching() {
        let query = "charon";
        let contents = "\
            No coins for Charon, unable to cross over worlds
            Dumped out in the river, found unworthy to pass now";
        assert_eq!(vec![] as Vec<&str>, search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive_ok() {
        let query = "Rope";
        let contents = "\
        I’m at the end of my rope
        Life seems so clear
        Noose securely fastened
        As death draws near";

        assert_eq!(
            vec!["I’m at the end of my rope"],
            search_case_insensitive(query, contents)
        )
    }
}
