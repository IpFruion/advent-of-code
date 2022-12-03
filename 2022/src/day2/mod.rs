//! Day 2 consists of doing `Rock, Paper or Scissors` with other elves.

use advent_of_code::errors::Result;

use self::round::Round;

pub mod round;
pub mod rps;
pub mod state;

/// The Solution for part 1 of day 2.
pub fn solution<S: AsRef<str>, I: Iterator<Item = S>>(lines: I) -> Result<usize> {
    lines
        .map(|line| {
            let line = line.as_ref();
            Round::try_from_players_line(line).map(|round| round.compete())
        })
        .sum()
}

/// The Solution for part 2 of day 2.
pub fn solution_pt2<S: AsRef<str>, I: Iterator<Item = S>>(lines: I) -> Result<usize> {
    lines
        .map(|line| {
            let line = line.as_ref();
            Round::try_from_decided_line(line).map(|round| round.compete())
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use advent_of_code::safe_lines;

    use super::{round::Round, rps::RPS, solution, solution_pt2};

    #[test]
    fn issue_with_rock_scissor() {
        let round = Round::new(RPS::Rock, RPS::Scissors);

        assert_eq!(round.compete(), 3)
    }

    #[test]
    fn page_example() {
        let lines = safe_lines("input/page_example_2.txt").unwrap();
        let points = solution(lines).unwrap();
        assert_eq!(points, 15)
    }

    #[test]
    fn page_example_2() {
        let lines = safe_lines("input/page_example_2.txt").unwrap();
        let points = solution_pt2(lines).unwrap();
        assert_eq!(points, 12)
    }
}
