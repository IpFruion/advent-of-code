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
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

#[cfg(test)]
mod tests {
    use advent_of_code::safe_lines;

    use crate::{
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day_10, day_11, day_12, day_13,
        day_14, day_15,
    };

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

    #[test]
    fn day_4_pt_1() {
        let lines = safe_lines("input/day_4.txt").unwrap();
        let score = day4::solution_pt1(lines).unwrap();
        println!("Day 4 Part 1: {}", score)
    }

    #[test]
    fn day_4_pt_2() {
        let lines = safe_lines("input/day_4.txt").unwrap();
        let score = day4::solution_pt2(lines).unwrap();
        println!("Day 4 Part 2: {}", score)
    }

    #[test]
    fn day_5_pt_1() {
        let lines = safe_lines("input/day_5.txt").unwrap();
        let values = day5::solution_pt1(lines).unwrap();
        println!("Day 5 Part 1: {}", values)
    }

    #[test]
    fn day_5_pt_2() {
        let lines = safe_lines("input/day_5.txt").unwrap();
        let values = day5::solution_pt2(lines).unwrap();
        println!("Day 5 Part 2: {}", values)
    }

    #[test]
    fn day_6_pt_1() {
        let mut lines = safe_lines("input/day_6.txt").unwrap();
        let first_line = lines.next().unwrap();
        let values = day6::solution(4, &first_line).unwrap();
        println!("Day 6 Part 1: {}", values)
    }

    #[test]
    fn day_6_pt_2() {
        let mut lines = safe_lines("input/day_6.txt").unwrap();
        let first_line = lines.next().unwrap();
        let values = day6::solution(14, &first_line).unwrap();
        println!("Day 6 Part 2: {}", values)
    }

    #[test]
    fn day_7_pt_1() {
        let lines = safe_lines("input/day_7.txt").unwrap();
        let values = day7::solution_pt1(lines).unwrap();
        println!("Day 7 Part 1: {}", values)
    }

    #[test]
    fn day_7_pt_2() {
        let lines = safe_lines("input/day_7.txt").unwrap();
        let values = day7::solution_pt2(lines).unwrap();
        println!("Day 7 Part 2: {}", values)
    }

    #[test]
    fn day_8_pt_1() {
        let lines = safe_lines("input/day_8.txt").unwrap();
        let values = day8::solution_pt1(lines).unwrap();
        println!("Day 8 Part 1: {}", values)
    }

    #[test]
    fn day_8_pt_2() {
        let lines = safe_lines("input/day_8.txt").unwrap();
        let values = day8::solution_pt2(lines).unwrap();
        println!("Day 8 Part 2: {}", values)
    }

    #[test]
    fn day_9_pt_1() {
        let lines = safe_lines("input/day_9.txt").unwrap();
        let values = day9::solution_pt1(lines).unwrap();
        println!("Day 9 Part 1: {}", values)
    }

    #[test]
    fn day_9_pt_2() {
        let lines = safe_lines("input/day_9.txt").unwrap();
        let values = day9::solution_pt2(lines).unwrap();
        println!("Day 9 Part 2: {}", values)
    }

    #[test]
    fn day_10_pt_1() {
        let lines = safe_lines("input/day_10.txt").unwrap();
        let values = day_10::solution_pt1(lines).unwrap();
        println!("Day 10 Part 1: {}", values)
    }

    #[test]
    fn day_10_pt_2() {
        let lines = safe_lines("input/day_10.txt").unwrap();
        let values = day_10::solution_pt2(lines).unwrap();
        println!("Day 10 Part 2: \n{}", values)
    }

    #[test]
    fn day_11_pt_1() {
        let lines = safe_lines("input/day_11.txt").unwrap();
        let values = day_11::solution_pt1(lines).unwrap();
        println!("Day 11 Part 1: \n{}", values)
    }

    #[test]
    fn day_11_pt_2() {
        let lines = safe_lines("input/day_11.txt").unwrap();
        let values = day_11::solution_pt2(lines).unwrap();
        println!("Day 11 Part 2: \n{}", values)
    }

    #[test]
    fn day_12_pt_1() {
        let lines = safe_lines("input/day_12.txt").unwrap();
        let values = day_12::solution_pt1(lines).unwrap();
        println!("Day 12 Part 1: \n{}", values)
    }

    #[test]
    fn day_12_pt_2() {
        let lines = safe_lines("input/day_12.txt").unwrap();
        let values = day_12::solution_pt2(lines).unwrap();
        println!("Day 12 Part 2: \n{}", values)
    }

    #[test]
    fn day_13_pt_1() {
        let lines = safe_lines("input/day_13.txt").unwrap();
        let values = day_13::solution_pt1(lines).unwrap();
        println!("Day 13 Part 1: \n{}", values)
    }

    #[test]
    fn day_13_pt_2() {
        let lines = safe_lines("input/day_13.txt").unwrap();
        let values = day_13::solution_pt2(lines).unwrap();
        println!("Day 13 Part 2: \n{}", values)
    }

    #[test]
    fn day_14_pt_1() {
        let lines = safe_lines("input/day_14.txt").unwrap();
        let values = day_14::solution_pt1(lines).unwrap();
        println!("Day 14 Part 1: \n{}", values)
    }

    #[test]
    fn day_14_pt_2() {
        let lines = safe_lines("input/day_14.txt").unwrap();
        let values = day_14::solution_pt2(lines).unwrap();
        println!("Day 14 Part 2: \n{}", values)
    }

    #[test]
    fn day_15_pt_1() {
        let lines = safe_lines("input/day_15.txt").unwrap();
        let values = day_15::solution_pt1(lines, 2_000_000).unwrap();
        println!("Day 15 Part 1: \n{}", values)
    }

    #[test]
    fn day_15_pt_2() {
        let lines = safe_lines("input/day_15.txt").unwrap();
        let values = day_15::solution_pt2(lines, 4_000_000).unwrap();
        println!("Day 15 Part 2: \n{}", values)
    }
}
