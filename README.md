# Nergal

Yet another grep analogue build with Rust. Since 0666.

# Prerequisites:

- `rustc 1.80.0` installed
- `cargo 1.80.0` installed

# Usage

1. Clone the repo
2. `$ cargo test` to see if the tests are passing
3. `$ cargo run -- string_to_lookup path_to_file` to run a search

> If you want to conduct a case-insensitive search, you have to set your environmental variable IGNORE_CASE to *anything*:
> 
> `$ IGNORE_CASE=value cargo run -- string_to_lookup path_to_file`
