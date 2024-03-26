use std::cmp::Reverse;
use std::collections::HashMap;

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

    pub fn get_result(&self) -> Vec<SubsequentCharStats> {
        let mut result: Vec<SubsequentCharStats> = self.characters.values().cloned().collect();
        result.sort_unstable_by_key(|item| (Reverse(item.count), item.character));
        return result;
    }
}
