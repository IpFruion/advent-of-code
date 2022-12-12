use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

use super::{
    cell::HeightCell,
    moves::{Direction, Pos},
};
use advent_of_code::errors::{Error, Result};

pub struct HeightMap {
    // top left is (0, 0) and bottom right is (usize, usize)
    grid: Vec<Vec<HeightCell>>,
    start_pos: Pos,
    end_pos: Pos,
    // height, width
    dims: (usize, usize),
}

impl HeightMap {
    fn get_neighbor(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        match dir {
            Direction::Up => Some(Pos {
                r: pos.r.checked_sub(1)?,
                c: pos.c,
            }),
            Direction::Down => Some(Pos {
                r: pos.r.checked_add(1).filter(|y| *y < self.dims.0)?,
                c: pos.c,
            }),
            Direction::Left => Some(Pos {
                r: pos.r,
                c: pos.c.checked_sub(1)?,
            }),
            Direction::Right => Some(Pos {
                r: pos.r,
                c: pos.c.checked_add(1).filter(|x| *x < self.dims.1)?,
            }),
        }
    }

    pub fn start(&self) -> &HeightCell {
        &self.grid[self.start_pos.r][self.start_pos.c]
    }

    pub fn get_cell(&self, pos: Pos) -> Option<&HeightCell> {
        Some(&self.grid[pos.r][pos.c])
    }

    pub fn print_path(&self, path: &Vec<Pos>) {
        let path_set: HashSet<&Pos> = HashSet::from_iter(path.iter());
        for r in 0..self.dims.0 {
            for c in 0..self.dims.1 {
                let pos: Pos = (r, c).into();
                if path_set.contains(&pos) {
                    print!("\x1b[95m{}\x1b[0m", self.grid[r][c],);
                } else {
                    print!("{}", self.grid[r][c],);
                }
            }
            println!();
        }
    }

    fn get_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        [
            self.get_neighbor(pos, Direction::Up),
            self.get_neighbor(pos, Direction::Down),
            self.get_neighbor(pos, Direction::Left),
            self.get_neighbor(pos, Direction::Right),
        ]
        .into_iter()
        .filter_map(|p| p)
    }

    pub fn from_lines<S: AsRef<str>, L: Iterator<Item = S>>(lines: &mut L) -> Result<Self> {
        let grid: Vec<Vec<HeightCell>> = lines
            .filter(|s| !s.as_ref().trim().is_empty())
            .map(|line| line.as_ref().trim().chars().map(|c| c.into()).collect())
            .collect();

        let start_pos: Pos = grid
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, c)| matches!(c, HeightCell::Start))
                    .map(|(c, _)| (r, c))
            })
            .ok_or::<Error>("Start Position Not Found".into())?
            .into();

        let end_pos: Pos = grid
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, c)| matches!(c, HeightCell::End))
                    .map(|(c, _)| (r, c))
            })
            .ok_or::<Error>("End Position Not Found".into())?
            .into();

        let dims = (
            grid.len(),
            grid.get(0)
                .ok_or::<Error>("Need at least 1 row".into())?
                .len(),
        );
        Ok(HeightMap {
            grid,
            start_pos,
            end_pos,
            dims,
        })
    }

    pub fn shortest_path(&self, elevation_diff: u32) -> Vec<Pos> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        heap.push(State {
            cost: 0,
            pos: self.start_pos,
        });
        distances.insert(self.start_pos, 0);

        let mut visited = HashSet::new();
        let mut edges = HashMap::new();
        while let Some(State { cost, pos }) = heap.pop() {
            let dist = distances[&pos];
            if pos == self.end_pos {
                break;
            }
            if cost > dist {
                continue;
            }
            // println!(
            //     "{} = {:?}, Cost: {}, Dist: {}",
            //     pos, self.grid[pos.r][pos.c], cost, dist
            // );
            let height = self.grid[pos.r][pos.c].elevation();
            for neighbor in self.get_neighbors(pos).filter(|v| !visited.contains(v)) {
                let neighbor_height = self.grid[neighbor.r][neighbor.c].elevation();
                let edge_cost = neighbor_height.checked_sub(height).unwrap_or(0);
                let mut thingy = elevation_diff;
                if neighbor_height == 0 {
                    thingy = 0;
                }
                let next = State {
                    cost: cost + edge_cost + thingy,
                    pos: neighbor,
                };
                if edge_cost <= 1
                    && next.cost < distances.get(&neighbor).copied().unwrap_or(u32::MAX)
                {
                    // println!(
                    //     "\tAdding Neighbor: {} = {:?}, Cost: {}, Dist: {}",
                    //     neighbor,
                    //     self.grid[neighbor.r][neighbor.c],
                    //     cost + new_distance,
                    //     new_distance
                    // );
                    // cost is added because that is the length of each edge added. Distance is
                    // just the edge distance between two points
                    distances.insert(neighbor, next.cost);
                    edges.insert(neighbor, pos);
                    heap.push(next);
                }
            }
            visited.insert(pos);
        }

        // println!("{:?}", heap);
        // for r in 0..self.dims.0 {
        //     for c in 0..self.dims.1 {
        //         print!(
        //             "{}",
        //             distances
        //                 .get(&(r, c).into())
        //                 .and_then(|v| char::from_u32(*v + 'a' as u32))
        //                 .unwrap_or('#'),
        //         );
        //     }
        //     println!();
        // }

        let mut path = Vec::new();
        path.push(self.end_pos);
        if edges.contains_key(&self.end_pos) {
            let mut cur = self.end_pos;
            while let Some(pos) = edges.get(&cur) {
                path.push(*pos);
                cur = *pos;
            }
        }
        path.reverse();

        // for (i, pos) in path.iter().enumerate() {
        //     println!("{} : {}", i, pos);
        // }

        path
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.dims.0 {
            for c in 0..self.dims.1 {
                write!(f, "{}", self.grid[r][c])?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    cost: u32,
    pos: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
