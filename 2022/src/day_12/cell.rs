pub enum Cell {
    Start,
    Elevation(char),
    End,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            c => Self::Elevation(c),
        }
    }
}
