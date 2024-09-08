use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Incorrect input: You were supposed to type in 2 command-like arguments: query and file path.\n\
        Terminating...");
    }

    let (query, file_path) = (&args[1], &args[2]);

    println!("Searching for query {query} in file {file_path}...")
}
