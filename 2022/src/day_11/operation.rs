use advent_of_code::errors::Error;
use std::hash::Hash;
use std::{borrow::Borrow, collections::HashMap, str::FromStr};

use super::test::Test;

#[derive(Debug)]
pub enum Token {
    Var(String),
    Value(i64),
}

impl Token {
    pub fn value<K: Borrow<str> + Hash + Eq>(&self, value_lookup: &HashMap<K, i64>) -> Option<i64> {
        match self {
            Self::Value(k) => Some(*k),
            Self::Var(s) => value_lookup.get(&s).copied(),
        }
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Ok(v) = s.parse() {
            return Ok(Self::Value(v));
        }
        if !s.chars().any(|c| !c.is_alphanumeric()) {
            return Ok(Self::Var(s.to_owned()));
        }
        Err(Error::InvalidParseError("Not a valid token".to_owned()))
    }
}

#[derive(Debug)]
pub enum Operation {
    Multiply(Token, Token),
    Add(Token, Token),
}

impl Operation {
    pub fn apply<K: Borrow<str> + Hash + Eq>(&self, variables: &HashMap<K, i64>) -> Option<i64> {
        match self {
            Self::Multiply(l, r) => Some(l.value(variables)? * r.value(variables)?),
            Self::Add(l, r) => Some(l.value(variables)? + r.value(variables)?),
        }
    }

    pub fn apply_mod<K: Borrow<str> + Hash + Eq>(
        &self,
        m: i64,
        variables: &HashMap<K, i64>,
    ) -> Option<i64> {
        match self {
            Self::Multiply(l, r) => {
                let l = l.value(variables)? % m;
                let r = r.value(variables)? % m;

                Some((l * r) % m)
            }
            Self::Add(l, r) => {
                let l = l.value(variables)? % m;
                let r = r.value(variables)? % m;

                Some((l + r) % m)
            }
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        //TODO: abstract the operator
        let mult_split = s.split_once('*');
        if let Some((first, second)) = mult_split {
            return Ok(Operation::Multiply(first.parse()?, second.parse()?));
        }
        let add_split = s.split_once('+');
        if let Some((first, second)) = add_split {
            return Ok(Operation::Add(first.parse()?, second.parse()?));
        }

        Err(Error::InvalidParseError("Invalid Operation".to_owned()))
    }
}
