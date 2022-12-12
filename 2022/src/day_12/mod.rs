use advent_of_code::errors::Result;

mod moves;
mod cell;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    // breadth first search for shortest path. easy pz
    let grid: Vec<Vec<char>> = lines.filter(|s| !s.as_ref().trim().is_empty())
        .map(|line| line.as_ref().trim().chars())
    todo!()
}
