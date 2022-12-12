use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use super::{operation::Operation, test::Test};
use advent_of_code::errors::{Error, Result};

#[derive(Debug)]
pub struct Monkey {
    pub index: usize,
    item_worry_levels: VecDeque<i64>,
    operation: Operation,
    pub test: Test,
}

impl Monkey {
    pub fn get_thrown(&mut self) -> Option<Vec<(i64, usize)>> {
        let mut variables = HashMap::new();
        let mut output = Vec::with_capacity(self.item_worry_levels.capacity());
        while let Some(item) = self.item_worry_levels.pop_front() {
            variables.insert("old", item);
            let new = self.operation.apply(&variables)? / 3;
            let to = self.test.test(new);
            output.push((new, to));
        }

        Some(output)
    }

    pub fn get_thrown_without_worry(&mut self, divisor: i64) -> Option<Vec<(i64, usize)>> {
        let mut variables = HashMap::new();
        let mut output = Vec::with_capacity(self.item_worry_levels.capacity());
        while let Some(item) = self.item_worry_levels.pop_front() {
            variables.insert("old", item);
            let new = self.operation.apply_mod(divisor, &variables)?;
            let to = self.test.test(new);
            output.push((new, to));
        }

        Some(output)
    }

    pub fn add_item(&mut self, item: i64) {
        self.item_worry_levels.push_back(item);
    }

    pub fn from_lines<S: AsRef<str>, L: Iterator<Item = S>>(lines: &mut L) -> Result<Self> {
        let need_field = || Error::InvalidParseError("Need field to make a monkey".to_owned());
        let index = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .trim()
            .split_whitespace()
            .last()
            .ok_or_else(need_field)?
            .split_once(':')
            .ok_or_else(need_field)?
            .0
            .parse()?;

        let item_worry_levels = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .trim()
            .split_once(':')
            .ok_or_else(need_field)?
            .1
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i64>().map_err(|e| e.into()))
            .collect::<Result<_>>()?;

        let operation = lines
            .next()
            .ok_or_else(need_field)?
            .as_ref()
            .trim()
            .split_once(' ')
            .ok_or_else(need_field)?
            .1
            .split_once('=')
            .ok_or_else(need_field)?
            .1
            .trim()
            .parse()?;

        let test = Test::from_lines(lines)?;

        Ok(Monkey {
            index,
            item_worry_levels,
            operation,
            test,
        })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M {}: {:?}", self.index, self.item_worry_levels)
    }
}
