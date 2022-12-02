#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub mod day1;
pub mod day2;
mod errors;

/// ignores an error when reading a line and returns early
pub fn safe_lines<S: AsRef<Path>>(filename: S) -> Result<impl Iterator<Item = String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines().map_while(|r| r.ok()))
}

#[cfg(test)]
mod tests {
    use crate::{day1, day2, safe_lines};

    #[test]
    fn day_1_pt_1() {
        let lines = safe_lines("input/day_1.txt").unwrap();
        let top_calories = day1::solution::<1, _, _>(lines).unwrap();
        println!("Day 1 Part 1: {}", top_calories)
    }

    #[test]
    fn day_1_pt_2() {
        let lines = safe_lines("input/day_1.txt").unwrap();
        let top_calories = day1::solution::<3, _, _>(lines).unwrap();
        println!("Day 1 Part 2: {}", top_calories)
    }

    #[test]
    fn day_2_pt_1() {
        let lines = safe_lines("input/day_2.txt").unwrap();
        let score = day2::solution(lines).unwrap();
        println!("Day 2 Part 1: {}", score)
    }

    #[test]
    fn day_2_pt_2() {
        let lines = safe_lines("input/day_2.txt").unwrap();
        let score = day2::solution_pt2(lines).unwrap();
        println!("Day 2 Part 2: {}", score)
    }
}
