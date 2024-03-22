use std::collections::HashMap;
use std::process;

use crate::char_stats::CharStats;

mod char_stats;

fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut all_char_stats: HashMap<char, CharStats> = HashMap::new();

    for character in input.chars() {
        let entry = all_char_stats
            .entry(character)
            .or_insert(CharStats::new(character));

        entry.count += 1;
    }

    let mut result: Vec<CharStats> = all_char_stats.into_values().collect();
    result.sort_by(|a, b| b.count.cmp(&a.count));

    println!("{:?}", result);
}
