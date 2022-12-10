use advent_of_code::errors::Result;

use direction::Direction;
use grid::Grid;
mod direction;
mod grid;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let grid = Grid::from_lines(lines)?;
    println!("{}", grid);

    let (height, width) = grid.dims();
    let edge_amount = 2 * (height + width) - 4;

    let mut visible = edge_amount;

    for r in 1..height - 1 {
        for c in 1..width - 1 {
            let pos = (r, c);
            let is_visible = grid.is_visible(pos, Direction::Up)
                || grid.is_visible(pos, Direction::Right)
                || grid.is_visible(pos, Direction::Down)
                || grid.is_visible(pos, Direction::Left);
            if is_visible {
                visible += 1;
            }
        }
    }

    Ok(visible)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let grid = Grid::from_lines(lines)?;
    println!("{}", grid);

    let (height, width) = grid.dims();
    let mut max_scenic_score = 0;

    for r in 0..height {
        for c in 0..width {
            let pos = (r, c);
            let scenic_score = grid.viewing_distance(pos, Direction::Up)
                * grid.viewing_distance(pos, Direction::Right)
                * grid.viewing_distance(pos, Direction::Down)
                * grid.viewing_distance(pos, Direction::Left);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    Ok(max_scenic_score)
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
30373
25512
65332
33549
35390
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.split('\n');
        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 21);
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.split('\n');
        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 8);
    }
}
