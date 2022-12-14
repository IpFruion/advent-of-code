use std::{collections::HashSet, fmt::Display};

use super::{direction::Direction, path::Path, point::Point};

pub struct SandProducer {
    entry: Point,
    current: Point,
    //TODO: optimize using BTreeSet
    blocked_points: HashSet<Point>,
    sand: Vec<Point>,
    abyss: usize,
    floor: Option<usize>,
}

impl SandProducer {
    pub fn new<P: Into<Point>>(point: P, paths: &Vec<Path>) -> Self {
        let mut blocked_points = HashSet::new();

        for p in paths.iter() {
            let k = p.filler();
            blocked_points.extend(k.0)
        }

        let max_val = paths
            .iter()
            .flat_map(|p| p.0.iter())
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .map(|p| p.y)
            .unwrap_or(0);
        let point = point.into();
        SandProducer {
            entry: point,
            current: point,
            blocked_points,
            sand: Vec::new(),
            abyss: max_val + 3,
            floor: None,
        }
    }

    pub fn new_with_floor<P: Into<Point>>(point: P, paths: &Vec<Path>) -> Self {
        let mut blocked_points = HashSet::new();

        for p in paths.iter() {
            let k = p.filler();
            blocked_points.extend(k.0)
        }

        let max_val = paths
            .iter()
            .flat_map(|p| p.0.iter())
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .map(|p| p.y)
            .unwrap_or(0);
        let point = point.into();
        SandProducer {
            entry: point,
            current: point,
            blocked_points,
            sand: Vec::new(),
            abyss: max_val + 3,       // if max is 9 then this is 12
            floor: Some(max_val + 2), // if max is 9 then this 11
        }
    }

    pub fn display<'a>(&'a self, widths: (usize, usize)) -> DisplayBox<'a> {
        DisplayBox {
            producer: self,
            widths,
        }
    }

    fn is_blocked_by_path(&self, p: &Point) -> bool {
        if let Some(floor) = self.floor {
            if p.y >= floor {
                return true;
            }
        }
        self.blocked_points.contains(p)
    }

    fn is_point_blocked(&self, p: &Point) -> bool {
        self.sand.binary_search(p).is_ok() || self.is_blocked_by_path(p)
    }

    fn has_reached_abyss(&self, p: &Point) -> bool {
        self.abyss < p.y
    }
}

impl Iterator for SandProducer {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_reached_abyss(&self.current) {
            return None;
        }
        // sand can't produce anymore because entry is blocked
        if self.is_point_blocked(&self.entry) {
            return None;
        }

        let mut new_current = self.current.clone();
        while !self.is_point_blocked(&new_current) && !self.has_reached_abyss(&new_current) {
            let mut tmp = Direction::Down.move_point(&new_current);
            if self.is_point_blocked(&tmp) {
                tmp = Direction::DownLeft.move_point(&new_current);
            }
            if self.is_point_blocked(&tmp) {
                tmp = Direction::DownRight.move_point(&new_current);
            }
            if self.is_point_blocked(&tmp) {
                break;
            }
            new_current = tmp;
        }
        if self.has_reached_abyss(&new_current) {
            self.current = new_current;
            return None;
        }

        let i = self.sand.binary_search(&new_current).unwrap_or_else(|e| e);
        self.sand.insert(i, new_current);

        self.current = self.entry.clone();
        Some(new_current)
    }
}

//TODO: with numbers i.e. rows and columns numbers
pub struct DisplayBox<'a> {
    producer: &'a SandProducer,
    widths: (usize, usize),
}

impl<'a> Display for DisplayBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_x = self
            .producer
            .entry
            .x
            .checked_sub(self.widths.0)
            .unwrap_or(0);
        let end_x = self.producer.entry.x + (self.widths.1);
        for y in self.producer.entry.y..=self.producer.entry.y + self.producer.abyss + 2 {
            for x in start_x..=end_x {
                let p: Point = (x, y).into();
                let c = match p {
                    p if self.producer.sand.contains(&p) => 'o',
                    p if p == self.producer.entry => '+',
                    p if self.producer.is_blocked_by_path(&p) => '#',
                    _ => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
