//! ## Part 1
//!
//! One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//!
//! Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//!
//! ## Part 2
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!     The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
//!     The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
//!     The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
//!     The fourth rucksack's compartments only share item type v.
//!     The fifth rucksack's compartments only share item type t.
//!     The sixth rucksack's compartments only share item type s.
//!
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//!     Lowercase item types a through z have priorities 1 through 26.
//!     Uppercase item types A through Z have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

use advent_of_code::errors::{Error, Result};

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<u64> {
    lines
        .map(|line| {
            let line = line.as_ref();
            let (first_half, second_half) = line.split_at(line.len() / 2);
            if first_half.len() != second_half.len() {
                return Err(Error::InvalidParseError(
                    "Line must be divisible by two for chars".to_owned(),
                ));
            }

            let v1 = encode_line(first_half)?;
            let v2 = encode_line(second_half)?;
            let v = decode_value(v1 & v2);

            Ok(v)
        })
        .sum()
}

/// This method will take a character and encode it into a [u128].
///
/// This encoding is defined as the following.
/// - If `97 <= c <= 122`
fn encode_char(c: char) -> Result<u64> {
    let c = c as u32;
    let i = match c {
        97..=122 => c - 97,
        65..=90 => c - 65 + 26,
        _ => {
            return Err(Error::InvalidParseError(
                "Not a valid encoded character".to_owned(),
            ))
        }
    };

    Ok(1 << i)
}

fn encode_line(line: &str) -> Result<u64> {
    line.chars().fold(Ok(0), |accum, c| {
        let accum = accum?;
        let i = encode_char(c)?;
        Ok(accum | i)
    })
}

fn decode_value(v: u64) -> u64 {
    let mut v = v;
    let mut index = 1;
    while v & 1 != 1 {
        v = v >> 1;
        // println!("\t{:#064b}", v);
        index += 1;
    }
    index
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<u64> {
    let mut encoded_lines = [0; 3];
    // (sum, lines, index)
    lines
        .fold(Ok((0, &mut encoded_lines, 0)), |accum, line| {
            let (mut sum, lines, mut index) = accum?;
            if index >= 3 {
                let k = lines.iter().fold(lines[0], |accum, e| accum & e);
                // println!("number {:#064b}", k);
                let value = decode_value(k);
                sum += value;
                index = 0;
                lines.fill(0);
                // println!("value: {}", value);
            }
            lines[index] = encode_line(line.as_ref())?;
            // println!("{:#064b}", lines[index]);
            Ok((sum, lines, index + 1))
        })
        .map(|(sum, lines, index)| {
            if index >= 3 {
                let k = lines.iter().fold(lines[0], |accum, e| accum & e);
                let value = decode_value(k);
                return sum + value;
            }
            sum
        })
}

#[cfg(test)]
mod tests {
    use advent_of_code::safe_lines;

    use super::{solution_pt1, solution_pt2};

    #[test]
    fn page_example_pt_1() {
        let lines = safe_lines("input/page_example_3.txt").unwrap();
        let v = solution_pt1(lines).unwrap();
        assert_eq!(v, 157)
    }

    #[test]
    fn page_example_pt_2() {
        let lines = safe_lines("input/page_example_3.txt").unwrap();
        let v = solution_pt2(lines).unwrap();
        assert_eq!(v, 70)
    }
}
