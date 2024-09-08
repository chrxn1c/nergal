use std::env;
use std::process;
use nergal::Config;
use nergal::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
       println!("Incorrect input: {err}\n\
       Terminating...");
        process::exit(1);
    });
    println!("Searching for query {} in file {}...", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(2);
    }
}
