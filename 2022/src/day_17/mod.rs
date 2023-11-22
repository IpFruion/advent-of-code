use std::time::{Duration, SystemTime};

use advent_of_code::errors::Result;

use self::{board::BoardIter, moves::MoveIter};

pub mod board;
mod bounds;
mod moves;
mod piece;
mod point;
mod range;
mod shape;

pub fn solution_pt1<S: AsRef<str>>(s: S, iterations: usize) -> Result<isize> {
    let move_iter: MoveIter = s.as_ref().parse()?;

    let seg = 100_000;

    let mut now = SystemTime::now();
    let board = BoardIter::new(move_iter);
    let top_level = board
        .enumerate()
        .inspect(|(i, l)| {
            if i % seg == 0 {
                let diff = now.elapsed().unwrap();
                let speed = diff.as_nanos() / seg as u128;
                let how_many_left = iterations - i;
                println!(
                    "{}: {} | {:?}, ttl: {:?}",
                    i,
                    l,
                    diff,
                    Duration::from_nanos((how_many_left as u128 * speed) as u64)
                );
                now = SystemTime::now();
            }
        })
        .skip(iterations)
        .map(|(_, l)| l)
        .next()
        .ok_or("Couldn't take anymore")?;
    Ok(top_level - 3)
}

#[cfg(test)]
mod tests {
    use super::solution_pt1;

    const PAGE_EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn page_example_1() {
        let actual = solution_pt1(PAGE_EXAMPLE, 2022).unwrap();
        assert_eq!(actual, 3068)
    }
}
