use std::{collections::VecDeque, fmt::Display, str::Chars};

use advent_of_code::errors::Result;

use super::r#move::Move;

#[derive(Debug)]
pub struct Cargo {
    stacks: Vec<VecDeque<char>>,
}

fn read_cell(c: &mut Chars) -> Option<char> {
    c.next()?; // [
    let k = c.next()?; // cell
    c.next()?; // ]
    Some(k)
}

impl Cargo {
    pub fn from_lines<S: AsRef<str>, I: Iterator<Item = S>>(lines: &mut I) -> Result<Self> {
        let mut stacks = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.as_ref();
            if line.is_empty() {
                break;
            }
            let mut index = 0;
            let mut chars = line.chars();
            while let Some(cell) = read_cell(&mut chars) {
                if stacks.len() <= index {
                    stacks.push(VecDeque::new());
                }
                if cell.is_alphabetic() {
                    stacks[index].push_front(cell)
                }
                // space between stacks
                if let None = chars.next() {
                    break;
                }
                index += 1;
            }
        }

        Ok(Cargo { stacks })
    }

    pub fn apply_move(&mut self, cargo_move: &Move) {
        if !cargo_move.is_valid(self.stacks.len()) {
            return;
        }
        let (from, to) = cargo_move.get_from_to_indices();
        let (from, to) = if from < to {
            let (from_slice, to_slice) = self.stacks.split_at_mut(to);
            let (_, from_slice) = from_slice.split_at_mut(from);
            (from_slice.first_mut(), to_slice.first_mut())
        } else {
            let (to_slice, from_slice) = self.stacks.split_at_mut(from);
            let (_, to_slice) = to_slice.split_at_mut(to);
            (from_slice.first_mut(), to_slice.first_mut())
        };
        if let (Some(from), Some(to)) = (from, to) {
            for _ in 0..cargo_move.amount() {
                if let Some(v) = from.pop_back() {
                    to.push_back(v);
                }
            }
        }
    }

    pub fn apply_move_keep_order(&mut self, cargo_move: &Move) {
        if !cargo_move.is_valid(self.stacks.len()) {
            return;
        }
        let (from, to) = cargo_move.get_from_to_indices();
        let (from, to) = if from < to {
            let (from_slice, to_slice) = self.stacks.split_at_mut(to);
            let (_, from_slice) = from_slice.split_at_mut(from);
            (from_slice.first_mut(), to_slice.first_mut())
        } else {
            let (to_slice, from_slice) = self.stacks.split_at_mut(from);
            let (_, to_slice) = to_slice.split_at_mut(to);
            (from_slice.first_mut(), to_slice.first_mut())
        };
        if let (Some(from), Some(to)) = (from, to) {
            let mut from_chars = VecDeque::with_capacity(cargo_move.amount());
            for _ in 0..cargo_move.amount() {
                if let Some(c) = from.pop_back() {
                    from_chars.push_front(c);
                }
            }
            to.extend(from_chars);
        }
    }

    pub fn get_tops(&self) -> String {
        let mut s = String::new();
        for stack in self.stacks.iter() {
            if let Some(c) = stack.get(stack.len() - 1) {
                s.push(*c);
            } else {
                s.push('#');
            }
        }
        s
    }
}

impl Display for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_stack = 0;
        for stack in self.stacks.iter() {
            if stack.len() > max_stack {
                max_stack = stack.len();
            }
        }

        for i in (0..max_stack).rev() {
            for stack in self.stacks.iter() {
                if let Some(cell) = stack.get(i) {
                    write!(f, "[{}]", cell)?;
                } else {
                    write!(f, "   ")?;
                }
                write!(f, " ")?;
            }
            writeln!(f, "")?;
        }

        for i in 0..self.stacks.len() {
            write!(f, " {}  ", i + 1)?;
        }
        Ok(())
    }
}
