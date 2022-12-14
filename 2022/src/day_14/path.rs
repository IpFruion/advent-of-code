use std::{fmt::Display, ops::Deref, str::FromStr};

use advent_of_code::errors::Error;

use super::point::Point;

#[derive(Debug)]
pub struct Path(pub Vec<Point>);

impl Path {
    /// Fills in the points inbetween the points in the path
    pub fn filler(&self) -> Path {
        let mut splices = Vec::new();

        let mut iter = self.windows(2).enumerate();

        while let Some((i, [p1, p2])) = iter.next() {
            let fill: Vec<Point> = p1.line_iter(*p2).skip(1).collect();
            if fill.len() > 0 {
                let i = i + 1;
                splices.push((i..i, fill));
            }
        }

        let mut points = self.0.clone();
        for (r, fill) in splices.into_iter().rev() {
            points.splice(r, fill);
        }

        Path(points)
    }
}

impl Deref for Path {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split("->");
        let mut points = Vec::new();

        while let Some(p) = splits.next() {
            let p = p.trim();
            if p.is_empty() {
                continue;
            }
            points.push(p.parse()?)
        }

        Ok(Path(points))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for p in self.0.iter() {
            buff.push_str(&format!("{}", p));
            buff.push_str(" -> ")
        }
        if buff.len() >= 4 {
            buff.truncate(buff.len() - 4);
        }
        write!(f, "{}", buff)
    }
}
