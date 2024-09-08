use nergal::run;
use nergal::Config;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!(
            "Incorrect input: {err}\n\
       Terminating..."
        );
        process::exit(1);
    });
    println!(
        "Searching for query {} in file {}...",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(2);
    }
}
