use std::process;
use std::time::SystemTime;

use crate::all_chars::AllChars;

mod all_chars;
mod char_stats;
mod subsequent_char_stats;
mod subsequent_chars;

fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut all_chars = AllChars::new();
    let characters: Vec<char> = input.chars().collect();
    let character_count = characters.len();

    all_chars.start_time = Some(SystemTime::now());

    for i in 0..character_count {
        // Count this character.
        let character = characters[i];
        let mut subsequent_character: Option<char> = None;

        // Count the subsequent character.
        let next_index = i + 1;
        if next_index < character_count {
            subsequent_character = Some(characters[next_index]);
        }

        all_chars.add_character(character, subsequent_character);
    }

    all_chars.end_time = Some(SystemTime::now());

    match all_chars.get_result() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Failed to generate result: {}", error),
    }
}
