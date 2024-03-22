use std::collections::HashMap;
use std::fmt;

use crate::subsequent_char_stats::SubsequentCharStats;

pub struct CharStats {
    pub character: char,
    pub count: u32,
    pub subsequent_characters: HashMap<char, SubsequentCharStats>,
}

impl CharStats {
    pub fn new(character: char) -> CharStats {
        CharStats {
            character,
            count: 0,
            subsequent_characters: HashMap::new(),
        }
    }
}

impl fmt::Debug for CharStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {}, subsequent: {:?}\n",
            self.character, self.count, self.subsequent_characters
        )
    }
}
