use std::cmp::Reverse;
use std::collections::HashMap;

use crate::char_stats::CharStats;
use crate::subsequent_char_stats::SubsequentCharStats;

pub struct AllChars {
    pub characters: HashMap<char, CharStats>,
}

impl AllChars {
    pub fn new() -> AllChars {
        AllChars {
            characters: HashMap::new(),
        }
    }

    pub fn add_character(&mut self, character: char, subsequent_character: Option<char>) {
        let entry = self
            .characters
            .entry(character)
            .or_insert(CharStats::new(character));

        entry.count += 1;

        match subsequent_character {
            Some(c) => {
                let subsequent_entry = entry
                    .subsequent_characters
                    .entry(c)
                    .or_insert(SubsequentCharStats::new(c));

                subsequent_entry.count += 1;
            }
            None => (),
        }
    }

    pub fn get_result(&self) -> Vec<CharStats> {
        let mut result: Vec<CharStats> = self.characters.values().cloned().collect();
        result.sort_unstable_by_key(|item| (Reverse(item.count), item.character));
        return result;
    }
}
