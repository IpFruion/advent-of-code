//! Defines a Round to play [RPS] in.
//!
//! This is defined as player1 vs player2 `= Round(p1, p2)`
use std::fmt::Display;

use advent_of_code::errors::{Error, Result};

use super::{rps::RPS, state::GameState};

/// The structure to house a Round of [RPS].
pub struct Round(RPS, RPS);

impl Round {
    /// Creates a new [Round] of [RPS].
    ///
    /// Example
    /// ```
    /// let round = Round::new(RPS::Rock, RPS::Scissors);
    ///
    /// ```
    pub fn new(player1: RPS, player2: RPS) -> Self {
        Round(player1, player2)
    }

    pub fn new_decided(player1: RPS, outcome: GameState) -> Self {
        let player2 = (-outcome).against(&player1);
        Round(player1, player2)
    }

    pub fn compete(&self) -> usize {
        // If player 1 wins then it is negated to turn it into a player2 loss and the points are
        // gathered
        self.1.points() + (-(self.0 + self.1)).points()
    }

    pub fn try_from_players_line(line: &str) -> Result<Self> {
        let (p1, p2) = split_line(line)?;
        Ok(Self(p1, p2))
    }

    pub fn try_from_decided_line(line: &str) -> Result<Self> {
        let (p1, outcome) = split_line(line)?;
        Ok(Self::new_decided(p1, outcome))
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P1: {:?}, P2: {:?}", self.0, self.1)
    }
}

fn split_line<'a, I1, I2>(line: &'a str) -> Result<(I1, I2)>
where
    I1: TryFrom<&'a str, Error = Error>,
    I2: TryFrom<&'a str, Error = Error>,
{
    let mut round_splits = line.trim().split(' ');
    let i1 = round_splits
        .next()
        .ok_or(Error::InvalidStruct(
            "Invalid Input One of Round".to_owned(),
        ))?
        .try_into()?;

    let i2 = round_splits
        .next()
        .ok_or(Error::InvalidStruct(
            "Invalid Input Two of Round".to_owned(),
        ))?
        .try_into()?;

    Ok((i1, i2))
}
