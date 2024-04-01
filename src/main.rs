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

    // Extract the provided file path.
    let file_path = args.next().unwrap_or_else(|| {
        eprintln!("No file path provided.");
        process::exit(1);
    });

    let input = std::fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut all_chars = AllChars::new();
    all_chars.count(input.chars().collect());

    match all_chars.get_result() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Failed to generate result: {}", error),
    }
}
