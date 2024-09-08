use std::error::Error;

use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("needed 2 command-line arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
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
    let mut found_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found_lines.push(line);
        }
    }

    found_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut found_lines = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            found_lines.push(line);
        }
    }

    found_lines
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
