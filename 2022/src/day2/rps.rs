//! Defines the game `Rock, Paper, Scissors` and the scores given for the game.
use std::ops::{Add, Neg};

use advent_of_code::errors::Error;

use super::state::GameState;

/// The move in the game `RPS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    /// Returns the points associated with each move.
    pub fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

/// Converts a character into a move associated with [RPS].
///
/// Valid characters are
/// - `A or X = Rock`
/// - `B or Y = Paper`
/// - `C or Z = Scissors`
impl TryFrom<char> for RPS {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(Error::InvalidStruct(
                "Not a valid Rock, Paper or Scissors".to_owned(),
            )),
        }
    }
}

/// Converts a string into a move associated with [RPS].
///
/// See [TryFrom char](#impl-TryFrom<char>-for-RPS) for more details.
impl TryFrom<&str> for RPS {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .ok_or(Error::InvalidStruct(
                "RPS string must not be empty".to_owned(),
            ))?
            .try_into()
    }
}

/// Compares two moves against each other to see who would win.
///
/// See [Ord](#impl-Ord-for-RPS) for more details.
impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares two [RPS] moves against each other to see which will win.
///
/// `self > other` implies that self `wins` over `other`
///
/// Example
/// ```
/// assert!(RPS::Rock > RPS::Scissors);
/// assert!(RPS::Paper < RPS::Scissors);
/// assert!(RPS::Rock < RPS::Paper);
///
/// ```
impl Ord for RPS {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            return std::cmp::Ordering::Equal;
        }
        match (self, other) {
            (Self::Rock, Self::Scissors) => std::cmp::Ordering::Greater,
            (Self::Scissors, Self::Paper) => std::cmp::Ordering::Greater,
            (Self::Paper, Self::Rock) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Less,
        }
    }
}

/// Negation of an [RPS] move is defined as the following.
///
/// Let move `a` be `Rock`. `-a` is the opposite of `a` defining `a > -a` or inversely `-a < a`.
/// This implies `-a` to be Scissors to make the above statement `true`.
///
/// Example
/// defining `Paper < Scissors = Paper = -Scissors`
/// ```
/// assert_eq!(RPS::Paper, -RPS::Scissors)
/// ```
impl Neg for &RPS {
    type Output = RPS;

    fn neg(self) -> Self::Output {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

impl Neg for RPS {
    type Output = RPS;

    fn neg(self) -> Self::Output {
        -(&self)
    }
}

impl Add for &RPS {
    type Output = GameState;

    fn add(self, rhs: Self) -> Self::Output {
        match self.cmp(rhs) {
            std::cmp::Ordering::Greater => GameState::Win,
            std::cmp::Ordering::Equal => GameState::Draw,
            std::cmp::Ordering::Less => GameState::Lose,
        }
    }
}

impl Add for RPS {
    type Output = GameState;

    fn add(self, rhs: Self) -> Self::Output {
        (&self) + (&rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::state::GameState;

    use super::RPS;

    #[test]
    fn test_game_state_comput() {
        let k = RPS::Rock;

        // this describes that Rock + Scissors => Rock Wins Against Scissors
        assert_eq!(GameState::Win, k + -k);
        // this describes that Scissors + Rock => Scissors Losses Against Rock
        assert_eq!(GameState::Lose, -k + k);
        // this describes that Rock + Paper => Rock Losses Against Paper
        assert_eq!(GameState::Lose, k + --k);
    }
}
