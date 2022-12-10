use std::{
    ops::{AddAssign, Sub},
    str::FromStr,
};

use advent_of_code::errors::Error;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
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
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();

        let dir = splits
            .next()
            .ok_or(Error::InvalidParseError("No direction".to_owned()))?
            .chars()
            .next()
            .ok_or(Error::InvalidParseError("No direction".to_owned()))?;
        let amount = i64::from_str(
            splits
                .next()
                .ok_or(Error::InvalidParseError("No amount".to_owned()))?,
        )?;
        match dir {
            'U' => Ok(Vector { x: 0, y: amount }),
            'R' => Ok(Vector { x: amount, y: 0 }),
            'L' => Ok(Vector { x: -amount, y: 0 }),
            'D' => Ok(Vector { x: 0, y: -amount }),
            _ => Err(Error::InvalidParseError("Invalid direction".to_owned())),
        }
    }
}

pub struct LineAlgorithm {
    current: Point,
    vec: Vector,
}

impl Iterator for LineAlgorithm {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.magnitude() == 0 {
            return None;
        }
        let dest = self.current + self.vec;
        let sx = self.current.x < dest.x;

        todo!()
    }
}
