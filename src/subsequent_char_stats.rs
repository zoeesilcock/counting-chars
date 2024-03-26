use std::fmt;

#[derive(Clone)]
pub struct SubsequentCharStats {
    pub character: char,
    pub count: u32,
}

impl SubsequentCharStats {
    pub fn new(character: char) -> SubsequentCharStats {
        SubsequentCharStats {
            character,
            count: 0,
        }
    }
}

impl fmt::Debug for SubsequentCharStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
