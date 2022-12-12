pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// (0, 0) is top left of the grid
pub struct Pos {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Pos {
    fn from(x: (usize, usize)) -> Self {}
}
