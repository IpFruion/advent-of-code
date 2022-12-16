use std::{fmt::Display, str::FromStr};

use advent_of_code::errors::Error;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhatten_distance_to(&self, p: &Point) -> isize {
        let x = p.x - self.x;
        let y = p.y - self.y;
        return x.abs() + y.abs();
    }

    pub fn manhatten_box(&self, p: &Point) -> [Point; 4] {
        let dist = self.manhatten_distance_to(p);
        [
            (self.x, self.y - dist).into(),
            (self.x + dist, self.y).into(),
            (self.x, self.y + dist).into(),
            (self.x - dist, self.y).into(),
        ]
    }
}

impl From<(isize, isize)> for Point {
    fn from(v: (isize, isize)) -> Self {
        Point { x: v.0, y: v.1 }
    }
}

impl FromStr for Point {
    type Err = Error;

    /// example: `x=2, y=18`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split(',');

        let x = splits
            .next()
            .ok_or(Error::InvalidParseError("No x value".to_owned()))?
            .split('=')
            .last()
            .ok_or(Error::InvalidParseError("No x value".to_owned()))?
            .parse()?;

        let y = splits
            .next()
            .ok_or(Error::InvalidParseError("No y value".to_owned()))?
            .split('=')
            .last()
            .ok_or(Error::InvalidParseError("No x value".to_owned()))?
            .parse()?;
        Ok(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
