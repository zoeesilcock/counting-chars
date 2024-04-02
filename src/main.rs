use std::env;
use std::process;

use crate::all_chars::AllChars;

mod all_chars;
mod char_stats;
mod subsequent_char_stats;
mod subsequent_chars;

fn main() {
    let mut args = env::args();

    // Skip the command name.
    args.next();

    let mut file_path = String::new();
    let mut ignore_case = false;

    // Extract CLI arguments.
    while let Some(arg) = args.next() {
        match arg {
            arg if arg == "-i" || arg == "--ignore-case" => {
                ignore_case = true;
            }
            arg => file_path = arg,
        }
    }

    // Read the input file.
    let input = std::fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    // Count the characters.
    let mut all_chars = AllChars::new(ignore_case);
    all_chars.count(input.chars().collect());

    // Output the results.
    match all_chars.get_result() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Failed to generate result: {}", error),
    }
}
