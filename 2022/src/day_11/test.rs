use advent_of_code::errors::{Error, Result};

#[derive(Debug)]
pub struct Test {
    pub divisor: u64,
    condition_true_id: usize,
    condition_false_id: usize,
}

impl Test {
    pub fn test(&self, worry_level: i64) -> usize {
        if worry_level % self.divisor as i64 == 0 {
            self.condition_true_id
        } else {
            self.condition_false_id
        }
    }

    pub fn from_lines<S: AsRef<str>, L: Iterator<Item = S>>(lines: &mut L) -> Result<Self> {
        let need_field = || Error::InvalidParseError("Need field for Test".to_owned());

        let divisor: u64 = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .split_whitespace()
            .last()
            .ok_or_else(need_field)?
            .parse()?;

        let condition_true_id = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .split_whitespace()
            .last()
            .ok_or_else(need_field)?
            .parse()?;

        let condition_false_id = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .split_whitespace()
            .last()
            .ok_or_else(need_field)?
            .parse()?;

        Ok(Test {
            divisor,
            condition_true_id,
            condition_false_id,
        })
    }
}
