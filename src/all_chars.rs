use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::SystemTime;

use crate::char_stats::CharStats;

pub struct AllChars {
    pub characters: HashMap<char, CharStats>,
    pub total_count: u32,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
}

impl AllChars {
    pub fn new() -> AllChars {
        AllChars {
            characters: HashMap::new(),
            total_count: 0,
            start_time: None,
            end_time: None,
        }
    }

    pub fn count(&mut self, input_characters: Vec<char>) {
        let character_count = input_characters.len();
        self.start_time = Some(SystemTime::now());

        for i in 0..character_count {
            // Count this character.
            let character = input_characters[i];
            let mut subsequent_character: Option<char> = None;

            // Count the subsequent character.
            let next_index = i + 1;
            if next_index < character_count {
                subsequent_character = Some(input_characters[next_index]);
            }

            self.add_character(character, subsequent_character);
        }

        self.end_time = Some(SystemTime::now());
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

        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => match end.duration_since(start) {
                Ok(duration) => write!(result, "Time elapsed: {:?}\n", duration)?,
                Err(error) => write!(result, "Failed to calculate elapsed time: {}\n", error)?,
            },
            _ => write!(result, "Start and/or end time missing.\n")?,
        }

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
