use std::ops::{AddAssign, Sub};

use super::{line::LineAlgorithm, vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn iter_line(&self, end: Point) -> LineAlgorithm {
        LineAlgorithm::new(*self, end)
    }
}

impl<T: Into<i64>> From<(T, T)> for Point {
    fn from(p: (T, T)) -> Self {
        Point {
            x: p.0.into(),
            y: p.1.into(),
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: (self.x - rhs.x) as f64,
            y: (self.y - rhs.y) as f64,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x.round() as i64;
        self.y += rhs.y.round() as i64;
    }
}
