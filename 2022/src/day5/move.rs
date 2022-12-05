use std::str::FromStr;

use advent_of_code::errors::Error;

#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl Move {
    pub fn is_valid(&self, stack_len: usize) -> bool {
        self.amount != 0
            && self.from != self.to
            && self.from > 0
            && self.to > 0
            && self.from - 1 < stack_len
            && self.to - 1 < stack_len
    }

    #[inline]
    pub fn get_from_to_indices(&self) -> (usize, usize) {
        (self.from - 1, self.to - 1)
    }

    #[inline]
    pub fn amount(&self) -> usize {
        self.amount
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(Error::InvalidParseError("empty string".to_owned()));
        }
        let mut white_space = s.split_whitespace();
        let ended_to_soon = || Error::InvalidParseError("parsing ended too soon".to_owned());
        // move
        white_space.next().ok_or_else(ended_to_soon)?;

        let amount = white_space.next().ok_or_else(ended_to_soon)?;
        let amount = usize::from_str(amount)?;

        // from
        white_space.next().ok_or_else(ended_to_soon)?;

        let from = white_space.next().ok_or_else(ended_to_soon)?;
        let from = usize::from_str(from)?;

        // to
        white_space.next().ok_or_else(ended_to_soon)?;

        let to = white_space.next().ok_or_else(ended_to_soon)?;
        let to = usize::from_str(to)?;

        Ok(Move { from, to, amount })
    }
}
