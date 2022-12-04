#![doc(
    html_logo_url = "http://clipart-library.com/new_gallery/1004202_christmas-tree-transparent-background-png.png"
)]
#![allow(dead_code)]

//! # 2022 Advent of Code!
//!
//! See `README.md` at top of repository for more information
//!
//!

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

#[cfg(test)]
mod tests {
    use advent_of_code::safe_lines;

    use crate::{day1, day2, day3};

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

    #[test]
    fn day_3_pt_1() {
        let lines = safe_lines("input/day_3.txt").unwrap();
        let score = day3::solution_pt1(lines).unwrap();
        println!("Day 3 Part 1: {}", score)
    }

    #[test]
    fn day_3_pt_2() {
        let lines = safe_lines("input/day_3.txt").unwrap();
        let score = day3::solution_pt2(lines).unwrap();
        println!("Day 3 Part 2: {}", score)
    }
}
