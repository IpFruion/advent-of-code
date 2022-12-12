use std::fmt::Display;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// (0, 0) is top left of the grid
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub r: usize,
    pub c: usize,
}

impl From<(usize, usize)> for Pos {
    fn from((r, c): (usize, usize)) -> Self {
        Pos { r, c }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}
