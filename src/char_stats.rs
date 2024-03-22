use std::fmt;

pub struct CharStats {
    pub character: char,
    pub count: u32,
}

impl CharStats {
    pub fn new(character: char) -> CharStats {
        CharStats {
            character,
            count: 0,
        }
    }
}

impl fmt::Debug for CharStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.character, self.count)
    }
}
