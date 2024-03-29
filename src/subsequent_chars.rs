use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;

use crate::subsequent_char_stats::SubsequentCharStats;

#[derive(Clone)]
pub struct SubsequentChars {
    pub characters: HashMap<char, SubsequentCharStats>,
}

impl SubsequentChars {
    pub fn new() -> SubsequentChars {
        SubsequentChars {
            characters: HashMap::new(),
        }
    }

    pub fn add_character(&mut self, character: char) {
        let entry = self
            .characters
            .entry(character)
            .or_insert(SubsequentCharStats::new(character));

        entry.count += 1;
    }

    pub fn get_result(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut character_list: Vec<SubsequentCharStats> =
            self.characters.values().cloned().collect();
        character_list.sort_unstable_by_key(|item| (Reverse(item.count), item.character));

        for item in character_list {
            write!(result, "{:?}: {}, ", item.character, item.count)?;
        }

        return Ok(result);
    }
}
