use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
       println!("Incorrect input: {err}\n\
       Terminating...");
        process::exit(1);
    });
    println!("Searching for query {} in file {}...", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("File path is not located.");
    println!("contents:\n\n {}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("needed 2 command-line arguments")
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
