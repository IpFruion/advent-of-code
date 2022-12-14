use super::point::Point;

pub enum Direction {
    Down,
    DownLeft,
    DownRight,
}

impl Direction {
    /// panics if point moves past zero (to the left)
    pub fn move_point(&self, p: &Point) -> Point {
        match self {
            Self::Down => Point { x: p.x, y: p.y + 1 },
            Self::DownLeft => Point {
                x: p.x - 1,
                y: p.y + 1,
            },
            Self::DownRight => Point {
                x: p.x + 1,
                y: p.y + 1,
            },
        }
    }
}
