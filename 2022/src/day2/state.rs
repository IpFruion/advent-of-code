use std::ops::Neg;

use advent_of_code::errors::Error;

use super::rps::RPS;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Win,
    Draw,
    Lose,
}

impl GameState {
    // Defines a function to gather that player1 <GameStates> the returned value i.e. player1
    // [Win]s against player2
    pub fn against(&self, player1: &RPS) -> RPS {
        let opposite = -player1;
        match self {
            Self::Draw => player1.clone(),
            Self::Win => opposite,
            Self::Lose => -opposite,
        }
    }

    pub fn points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl Neg for GameState {
    type Output = GameState;

    fn neg(self) -> Self::Output {
        match self {
            Self::Win => Self::Lose,
            Self::Draw => Self::Draw,
            Self::Lose => Self::Win,
        }
    }
}

impl TryFrom<char> for GameState {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(Error::InvalidStruct(
                "Not a valid Rock, Paper or Scissors".to_owned(),
            )),
        }
    }
}

impl TryFrom<&str> for GameState {
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
