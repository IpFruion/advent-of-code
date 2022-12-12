use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum HeightCell {
    Start,
    Elevation(char),
    End,
}

impl HeightCell {
    pub fn elevation(&self) -> u32 {
        match self {
            Self::Start => 0,
            Self::End => 'z' as u32 - 'a' as u32,
            // a starts at 1
            Self::Elevation(c) => *c as u32 - 'a' as u32,
        }
    }
}

impl Display for HeightCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => 'S',
                Self::End => 'E',
                Self::Elevation(c) => *c,
            }
        )
    }
}

impl From<char> for HeightCell {
    fn from(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            c => Self::Elevation(c),
        }
    }
}
