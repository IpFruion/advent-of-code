//! Cargo crates for elves.
//! ## Part 1
//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//!
//! ```
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//!```

use advent_of_code::errors::Result;

use crate::day5::r#move::Move;

use self::cargo::Cargo;
mod cargo;
mod r#move;

pub fn solution_pt1<S: AsRef<str>, I: Iterator<Item = S>>(mut lines: I) -> Result<String> {
    // Parse cargo
    let mut cargo = Cargo::from_lines(&mut lines)?;
    println!("{}", cargo);
    // Parse moves
    // Apply moves
    while let Some(line) = lines.next() {
        let line = line.as_ref().trim();
        if line.is_empty() {
            break;
        }
        let m: Move = line.parse()?;
        cargo.apply_move(&m);
    }
    // Gather top cargo
    Ok(cargo.get_tops())
}

pub fn solution_pt2<S: AsRef<str>, I: Iterator<Item = S>>(mut lines: I) -> Result<String> {
    // Parse cargo
    let mut cargo = Cargo::from_lines(&mut lines)?;
    println!("{}", cargo);
    // Parse moves
    // Apply moves
    while let Some(line) = lines.next() {
        let line = line.as_ref().trim();
        if line.is_empty() {
            break;
        }
        let m: Move = line.parse()?;
        cargo.apply_move_keep_order(&m);
    }
    // Gather top cargo
    Ok(cargo.get_tops())
}

#[cfg(test)]
mod tests {

    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn page_example_1() {
        let mut lines = PAGE_EXAMPLE.split('\n');
        // get rid of empty new line
        lines.next();

        let res = solution_pt1(lines).unwrap();

        assert_eq!(res, "CMZ".to_owned());
    }

    #[test]
    fn page_example_2() {
        let mut lines = PAGE_EXAMPLE.split('\n');
        // get rid of empty new line
        lines.next();

        let res = solution_pt2(lines).unwrap();

        assert_eq!(res, "MCD".to_owned());
    }
}
