use std::fmt::Display;

use super::{bounds::Bounds, point::Point};

#[derive(Debug)]
pub struct Shape {
    points: Vec<Point>,
    //(bottom_left, top_right) top_right isn't inclusive i.e. if spans accross 1 value it would be
    //1
    bounds: Bounds,
}

impl Shape {
    pub fn shift(&mut self, by: &Point) {
        for p in self.points.iter_mut() {
            p.x += by.x;
            p.y += by.y;
        }
        self.bounds.shift(&by)
    }

    pub fn contains(&self, p: &Point) -> bool {
        // this is for the quicker check
        if !self.bounds.contains(p) {
            return false;
        }
        self.points.contains(p)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    #[inline]
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

impl FromIterator<Point> for Shape {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let points: Vec<Point> = iter.into_iter().collect();
        let bounds = points
            .iter()
            .collect::<Option<Bounds>>()
            .unwrap_or_default();

        Shape { points, bounds }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - [", self.bounds)?;
        for p in self.points.iter() {
            write!(f, "{}, ", p)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
