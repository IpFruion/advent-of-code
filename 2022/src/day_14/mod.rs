use std::{collections::HashSet, thread::sleep, time::Duration};

use advent_of_code::errors::Result;

use crate::day_14::point::Point;

use self::{path::Path, sand::SandProducer};

mod direction;
mod path;
mod point;
mod sand;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let paths: Vec<Path> = lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|l| l.as_ref().parse())
        .collect::<Result<_>>()?;

    let mut sand_iter = SandProducer::new((500, 0), &paths);

    let display_width = (20, 100);
    println!("{}", sand_iter.display(display_width));

    let mut count = 0;
    while let Some(_) = sand_iter.next() {
        // println!("{}", p);
        // println!("{}", sand_iter.display(display_width));
        count += 1;
    }

    Ok(count)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let paths: Vec<Path> = lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|l| l.as_ref().parse())
        .collect::<Result<_>>()?;

    let mut sand_iter = SandProducer::new_with_floor((500, 0), &paths);

    let display_width = (20, 100);
    // println!("{}", sand_iter.display(display_width));

    let mut count = 0;
    while let Some(_) = sand_iter.next() {
        // println!("{}", p);
        // print!("\x1B[2J\x1B[1;1H");
        // println!("{}", count);
        // println!("{}", sand_iter.display(display_width));
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 24)
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 93)
    }
}
