use advent_of_code::errors::Result;

use self::grid::HeightMap;

mod cell;
mod grid;
mod moves;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(mut lines: L) -> Result<usize> {
    // breadth first search for shortest path. easy pz
    let height_map = HeightMap::from_lines(&mut lines)?;
    println!("{}", height_map);

    let path = height_map.shortest_path(1);
    height_map.print_path(&path);

    Ok(path.len() - 1)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(mut lines: L) -> Result<usize> {
    let height_map = HeightMap::from_lines(&mut lines)?;
    println!("{}", height_map);

    let start_height = height_map.start().elevation();
    let mut path = height_map.shortest_path(1);
    let i = path
        .iter()
        .enumerate()
        .find_map(|(i, pos)| {
            let elevation = height_map.get_cell(*pos).unwrap().elevation();
            if elevation == start_height {
                None
            } else {
                Some(i)
            }
        })
        .unwrap()
        - 1;

    path.drain(0..i);
    height_map.print_path(&path);

    Ok(path.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 31)
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 29)
    }
}
