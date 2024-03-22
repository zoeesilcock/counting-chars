use std::cmp::Reverse;
use std::collections::HashMap;
use std::process;

use crate::char_stats::CharStats;
use crate::subsequent_char_stats::SubsequentCharStats;

mod char_stats;
mod subsequent_char_stats;

fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut all_char_stats: HashMap<char, CharStats> = HashMap::new();
    let characters: Vec<char> = input.chars().collect();
    let character_count = characters.len();

    for i in 0..character_count {
        // Count this character.
        let character = characters[i];
        let entry = all_char_stats
            .entry(character)
            .or_insert(CharStats::new(character));

        entry.count += 1;

        // Count the subsequent character.
        let next_index = i + 1;
        if next_index < character_count {
            let next_character = characters[next_index];
            let subsequent_entry = entry
                .subsequent_characters
                .entry(next_character)
                .or_insert(SubsequentCharStats::new(next_character));

            subsequent_entry.count += 1;
        }
    }

    let mut result: Vec<CharStats> = all_char_stats.into_values().collect();
    result.sort_unstable_by_key(|item| (Reverse(item.count), item.character));

    println!("{:?}", result);
}
