use std::{
    ops::{Div, Sub},
    str::FromStr,
};

use advent_of_code::errors::Error;

use super::line::VectorIter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn iter(&self) -> VectorIter {
        VectorIter::new(*self)
    }

    pub fn abs(&self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn unit(&self) -> Vector {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Vector { x: 0.0, y: 0.0 };
        }
        Vector {
            x: (self.x / mag),
            y: (self.y / mag),
        }
    }

    pub fn round(&self) -> Vector {
        Vector {
            x: self.x.round(),
            y: self.y.round(),
        }
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
        let amount = amount as f64;
        match dir {
            'U' => Ok(Vector { x: 0.0, y: amount }),
            'R' => Ok(Vector { x: amount, y: 0.0 }),
            'L' => Ok(Vector { x: -amount, y: 0.0 }),
            'D' => Ok(Vector { x: 0.0, y: -amount }),
            _ => Err(Error::InvalidParseError("Invalid direction".to_owned())),
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        Vector {
            x: if rhs.x == 0.0 { 0.0 } else { self.x / rhs.x },
            y: if rhs.y == 0.0 { 0.0 } else { self.y / rhs.y },
        }
    }
}
