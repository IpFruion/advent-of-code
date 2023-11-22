use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    ops::Range,
};

use crate::day_17::{moves::Push, shape::Shape};

use super::piece::PieceSpawner;

pub struct BoardIter<I: Iterator<Item = Push>> {
    top_level: usize,
    leading_edges: Vec<VecDeque<Range<isize>>>,
    spawner: PieceSpawner,
    moves_iter: I,
}

impl<I: Iterator<Item = Push>> BoardIter<I> {
    pub fn new(moves_iter: I) -> Self {
        BoardIter {
            top_level: 0,
            // leading edge is the ground for now
            leading_edges: (0..7).map(|_| VecDeque::from([0..1])).collect(),
            spawner: PieceSpawner::new(),
            moves_iter,
        }
    }

    fn with_shape<'a>(&'a self, shape: &'a Shape) -> DisplayBoard<'a, I> {
        DisplayBoard { board: self, shape }
    }

    /// this happens directly after a shape has moved
    pub fn is_blocked(&self, shape: &Shape) -> bool {
        // println!("\t{}", shape);
        // check the edges
        for p in shape.iter() {
            if let Some(edges) = self.leading_edges.get(p.x as usize) {
                for r in edges.iter() {
                    if r.end <= p.y {
                        break;
                    }
                    if r.contains(&p.y) {
                        return true;
                    }
                }
            } else {
                // case where we are reaching beyond bounds
                return true;
            }
        }
        return false;
    }

    /// this happens directly after a shape has been moved back above
    pub fn fill_in_shape(&mut self, shape: &Shape) {
        let mut current_top = self.leading_edges[self.top_level].front().unwrap().end;
        // println!("filling in: \t{}", shape);
        for p in shape.iter() {
            let edges = &mut self.leading_edges[p.x as usize];
            let mut insert_ledge = None;
            for (i, r) in edges.iter_mut().enumerate() {
                let diff = p.y + 1 - r.end;
                if diff > 0 {
                    if diff.abs() == 1 {
                        r.end = p.y + 1;
                    } else {
                        insert_ledge = Some(i);
                    }
                    break;
                }
            }
            if let Some(l) = insert_ledge {
                edges.insert(l, p.y..(p.y + 1));
            }
            if let Some(first) = edges.front() {
                if first.end > current_top {
                    self.top_level = p.x as usize;
                    current_top = first.end;
                }
            }
        }
    }
}

impl<I: Iterator<Item = Push>> Iterator for BoardIter<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{}", self);
        // ground is at zero
        let current_top = self.leading_edges[self.top_level].front().map(|r| r.end)?;
        let piece = self.spawner.next()?;
        let mut bounds = piece.bounds();
        bounds.shift(&(2, current_top + 3).into());
        while let Some(m) = self.moves_iter.next() {
            // println!("\t{}", self);
            // println!("\t{}", bounds);
            bounds.shift(&m.shift_amount());
            //not inclusive for the bounds
            if bounds.bottom_right().x > 7 || bounds.bottom_left().x < 0 {
                bounds.shift(&(-m).shift_amount());
                continue;
            }
            if bounds.bottom_left().y > current_top {
                // still above top
                continue;
            }
            match m {
                Push::Up => {}
                Push::Down => {
                    let shape = bounds.shape(&piece);
                    // let now = SystemTime::now();
                    if self.is_blocked(&shape) {
                        // reverse and solidfy
                        bounds.shift(&(-m).shift_amount());
                        let shape = bounds.shape(&piece);
                        self.fill_in_shape(&shape);
                        break;
                    }
                    // if let Ok(e) = now.elapsed() {
                    //     if e.as_micros() > 20 {
                    //         println!("{:?}", e);
                    //     }
                    // }
                }
                Push::Right | Push::Left => {
                    let shape = bounds.shape(&piece);
                    if self.is_blocked(&shape) {
                        // reverse and skip
                        bounds.shift(&(-m).shift_amount());
                        continue;
                    }
                }
            }
        }
        // println!("{:?}", self);

        self.leading_edges[self.top_level].front().map(|r| r.end)
    }
}

impl<I: Iterator<Item = Push>> Display for BoardIter<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let top = self.top_level.unwrap_or(0) + 4;
        // for y in (0..top).rev() {
        //     for x in 0..7 {
        //         let p: Point = (x, y).into();
        //         if self.rocks.iter().find(|k| p.eq(k)).is_some() {
        //             write!(f, "#")?;
        //         } else {
        //             write!(f, ".")?;
        //         }
        //     }
        //     writeln!(f)?;
        // }
        // for _ in 0..7 {
        //     write!(f, "-")?;
        // }
        // writeln!(f)?;
        //
        // Ok(())
        let mut s = String::new();
        for i in self.leading_edges.iter() {
            s.push_str(&format!("{:?}, ", i.front().unwrap()))
        }
        if s.len() >= 2 {
            s.truncate(s.len() - 2);
        }
        write!(f, "{}: [{}]", self.top_level, s)
    }
}

impl<I: Iterator<Item = Push>> Debug for BoardIter<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.top_level)?;
        for edges in self.leading_edges.iter() {
            writeln!(f, "{:?}", edges)?;
        }
        Ok(())
    }
}

pub struct DisplayBoard<'a, I: Iterator<Item = Push>> {
    board: &'a BoardIter<I>,
    shape: &'a Shape,
}

impl<'a, I: Iterator<Item = Push>> Display for DisplayBoard<'a, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let top = self.shape.bounds().top_right().y + 3;
        // for y in (0..top).rev() {
        //     for x in 0..7 {
        //         let p: Point = (x, y).into();
        //         if self.board.rocks.iter().find(|k| p.eq(k)).is_some() {
        //             write!(f, "#")?;
        //         } else if self.shape.iter().find(|k| p.eq(k)).is_some() {
        //             write!(f, "@")?;
        //         } else {
        //             write!(f, ".")?;
        //         }
        //     }
        //     writeln!(f)?;
        // }
        // for _ in 0..7 {
        //     write!(f, "-")?;
        // }
        // writeln!(f)?;
        // Ok(())
        Ok(())
    }
}
