use std::{ops::Neg, str::FromStr};

use advent_of_code::errors::Error;

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub enum Push {
    Left,
    Right,
    Down,
    Up,
}

impl Push {
    pub fn shift_amount(&self) -> Point {
        match self {
            Push::Left => (-1, 0),
            Push::Right => (1, 0),
            Push::Down => (0, -1),
            Push::Up => (0, 1),
        }
        .into()
    }
}

impl TryFrom<char> for Push {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '<' => Push::Left,
            '>' => Push::Right,
            _ => return Err(Error::InvalidParseError("not a correct move".to_owned())),
        })
    }
}

impl Neg for Push {
    type Output = Push;

    fn neg(self) -> Self::Output {
        match self {
            Push::Left => Push::Right,
            Push::Right => Push::Left,
            Push::Up => Push::Down,
            Push::Down => Push::Up,
        }
    }
}

pub struct MoveIter {
    cur: usize,
    is_down: bool,
    moves: Vec<Push>,
}

impl Iterator for MoveIter {
    type Item = Push;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.moves.len() {
            self.cur = 0;
        }
        if self.is_down {
            self.is_down = false;
            return Some(Push::Down);
        }
        if let Some(p) = self.moves.get(self.cur) {
            self.cur += 1;
            self.is_down = true;
            return Some(*p);
        }
        None
    }
}

impl FromStr for MoveIter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .trim()
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()?;

        Ok(MoveIter {
            cur: 0,
            is_down: false,
            moves,
        })
    }
}
