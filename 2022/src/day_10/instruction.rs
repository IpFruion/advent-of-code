use std::str::FromStr;

use advent_of_code::errors::Error;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split_whitespace();
        let i = splits
            .next()
            .ok_or(Error::InvalidParseError("Need an instruction".to_owned()))?;
        let inc = match i {
            "noop" => Instruction::Noop,
            "addx" => {
                let v = splits
                    .next()
                    .ok_or(Error::InvalidParseError("No value in addx".to_owned()))?;
                Instruction::AddX(i64::from_str(v)?)
            }
            _ => return Err(Error::InvalidParseError("unknown op".to_owned())),
        };

        Ok(inc)
    }
}
