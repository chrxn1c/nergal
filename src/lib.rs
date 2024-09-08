use std::error::Error;

use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("needed 2 command-line arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found_lines.push(line);
        }
    }

    found_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Charon";
        let contents = "\
            No coins for Charon, unable to cross over worlds
            Dumped out in the river, found unworthy to pass now";
        assert_eq!(
            vec!["No coins for Charon, unable to cross over worlds"],
            search(query, contents)
        );
    }
}
