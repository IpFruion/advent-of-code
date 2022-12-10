use std::collections::HashSet;

use advent_of_code::errors::Result;

use self::line::SimpleLineIter;
use self::point::Point;
use self::vector::Vector;

mod line;
mod point;
mod vector;

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let mut head: Point = (0, 0).into();
    let mut tail: Point = head.clone();
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in lines.filter(|l| !l.as_ref().trim().is_empty()) {
        let move_vec: Vector = line.as_ref().parse()?;
        // println!("Move: {:?}", move_vec);
        for _ in 0..move_vec.magnitude().round() as usize {
            head += move_vec.unit();
            // println!("\tHead: {:?} | Tail: {:?}", head, tail);
            let diff = head - tail;
            // println!("\tDiff: {:?}, {}", diff, diff.magnitude());
            if diff.magnitude() >= 2.0 {
                tail += (diff / diff.abs()).round();
                visited.insert(tail);
                // println!("\t\t Tail: {:?}", tail);
            }
        }
    }

    Ok(visited.len())
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let mut head: Point = (0, 0).into();
    let mut tails: Vec<Point> = Vec::from([head; 9]);
    let mut visited = HashSet::new();
    visited.insert(head);

    for line in lines.filter(|l| !l.as_ref().trim().is_empty()) {
        let move_vec: Vector = line.as_ref().parse()?;
        // println!("Move: {:?}", move_vec);
        for _ in 0..move_vec.magnitude().round() as usize {
            head += move_vec.unit();
            // println!("\tHead: {:?} | Tail: {:?}", head, tails);
            let mut cur = head;
            for (i, t) in tails.iter_mut().enumerate() {
                let diff = cur - *t;
                if diff.magnitude() >= 2.0 {
                    *t += (diff / diff.abs()).round();
                    if i == 8 {
                        visited.insert(*t);
                    }
                    // println!("\t\t TailUpdate: {} -> {:?}", i, t);
                }
                cur = *t;
            }
        }
    }

    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    const PAGE_EXAMPLE_2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn page_example_1() {
        let lines = PAGE_EXAMPLE.lines();
        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 13)
    }

    #[test]
    fn page_example_2() {
        let lines = PAGE_EXAMPLE_2.lines();
        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 36)
    }
}
