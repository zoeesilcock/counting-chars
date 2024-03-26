use std::fmt;

use crate::subsequent_chars::SubsequentChars;

#[derive(Clone)]
pub struct CharStats {
    pub character: char,
    pub count: u32,
    pub subsequent_characters: SubsequentChars,
}

impl CharStats {
    pub fn new(character: char) -> CharStats {
        CharStats {
            character,
            count: 0,
            subsequent_characters: SubsequentChars::new(),
        }
    }
}

impl fmt::Debug for CharStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {}, subsequent: {:?}\n",
            self.character,
            self.count,
            self.subsequent_characters.get_result()
        )
    }
}
