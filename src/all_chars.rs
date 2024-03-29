use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;

use crate::char_stats::CharStats;

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
            Some(c) => entry.subsequent_characters.add_character(c),
            None => (),
        }
    }

    pub fn get_result(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut character_list: Vec<CharStats> = self.characters.values().cloned().collect();
        character_list.sort_unstable_by_key(|item| (Reverse(item.count), item.character));

        for item in character_list {
            write!(
                result,
                "{:?}: {}, subsequent: [{}]\n",
                item.character,
                item.count,
                item.subsequent_characters.get_result()?
            )?;
        }

        return Ok(result);
    }
}
