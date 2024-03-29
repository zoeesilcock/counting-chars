use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::SystemTime;

use crate::char_stats::CharStats;

pub struct AllChars {
    pub characters: HashMap<char, CharStats>,
    pub total_count: u32,
}

impl AllChars {
    pub fn new() -> AllChars {
        AllChars {
            characters: HashMap::new(),
            total_count: 0,
        }
    }

    pub fn add_character(&mut self, character: char, subsequent_character: Option<char>) {
        let entry = self
            .characters
            .entry(character)
            .or_insert(CharStats::new(character));

        entry.count += 1;
        self.total_count += 1;

        match subsequent_character {
            Some(c) => entry.subsequent_characters.add_character(c),
            None => (),
        }
    }

    pub fn get_result(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut character_list: Vec<CharStats> = self.characters.values().cloned().collect();
        character_list.sort_unstable_by_key(|item| (Reverse(item.count), item.character));

        write!(result, "Total character count: {}\n", self.total_count)?;

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
