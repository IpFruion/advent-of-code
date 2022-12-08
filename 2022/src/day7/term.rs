use advent_of_code::errors::Error;
use std::str::FromStr;

pub enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        let cmd_to_short = || Error::InvalidParseError("Command to short".to_owned());
        let k = splits.next().ok_or_else(cmd_to_short)?;
        if k.trim() != "$" {
            return Err(Error::InvalidParseError("Command starts with $".to_owned()));
        }

        match splits.next().ok_or_else(cmd_to_short)? {
            "ls" => Ok(Self::Ls),
            "cd" => {
                let dir = splits.next().ok_or_else(cmd_to_short)?;
                Ok(Self::Cd(dir.trim().to_owned()))
            }
            _ => Err(Error::InvalidParseError("Invalid command".to_owned())),
        }
    }
}
