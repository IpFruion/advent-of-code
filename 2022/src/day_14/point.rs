use std::{fmt::Display, str::FromStr};

use advent_of_code::errors::Error;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Point {
    pub fn line_iter(&self, to: Point) -> LineAlgorithm {
        LineAlgorithm::new(*self, to)
    }
}

impl From<(usize, usize)> for Point {
    fn from(v: (usize, usize)) -> Self {
        Point { x: v.0, y: v.1 }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split(',');

        let x = splits
            .next()
            .ok_or(Error::InvalidParseError("No x value".to_owned()))?
            .parse()?;

        let y = splits
            .next()
            .ok_or(Error::InvalidParseError("No y value".to_owned()))?
            .parse()?;
        Ok(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// The Bresenham's Line aglorithm as an iterator of points
///
/// See [https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm]
/// Doesn't currently include the destination point sadly might fix later
pub struct LineAlgorithm {
    current: Point,
    dest: Point,
    delta: (isize, isize),
    increment: (isize, isize),
    error: isize,
}

impl LineAlgorithm {
    pub fn new(start: Point, end: Point) -> LineAlgorithm {
        let x = end
            .x
            .checked_sub(start.x)
            .unwrap_or_else(|| start.x - end.x) as isize;
        let y = end
            .y
            .checked_sub(start.y)
            .unwrap_or_else(|| start.y - end.y) as isize;
        let delta = (x, -y);
        let error = delta.0 + delta.1;
        LineAlgorithm {
            current: start,
            dest: end,
            delta,
            increment: (
                if start.x < end.x { 1 } else { -1 },
                if start.y < end.y { 1 } else { -1 },
            ),
            error,
        }
    }
}

impl Iterator for LineAlgorithm {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.dest {
            return None;
        }
        let item = self.current.clone();
        let e2 = 2 * self.error;
        if e2 >= self.delta.1 {
            if self.current.x == self.dest.x {
                return None;
            }
            self.error += self.delta.1;
            if self.increment.0 < 0 {
                self.current.x = self
                    .current
                    .x
                    .checked_sub(self.increment.0.abs() as usize)
                    .unwrap_or(0);
            } else {
                self.current.x += self.increment.0 as usize;
            }
        }
        if e2 <= self.delta.0 {
            if self.current.y == self.dest.y {
                return None;
            }
            self.error += self.delta.0;
            if self.increment.1 < 0 {
                self.current.y = self
                    .current
                    .y
                    .checked_sub(self.increment.1.abs() as usize)
                    .unwrap_or(0);
            } else {
                self.current.y += self.increment.1 as usize;
            }
        }
        Some(item)
    }
}
