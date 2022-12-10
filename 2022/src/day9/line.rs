use super::{point::Point, vector::Vector};

/// The Bresenham's Line aglorithm as an iterator of points
///
/// See [https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm]
pub struct LineAlgorithm {
    current: Point,
    dest: Point,
    delta: Point,
    increment: Point,
    error: i64,
}

impl LineAlgorithm {
    pub fn new(start: Point, end: Point) -> LineAlgorithm {
        let delta: Point = ((end.x - start.x).abs(), -(end.y - start.y).abs()).into();
        let error = delta.x + delta.y;
        LineAlgorithm {
            current: start,
            dest: end,
            delta,
            increment: (
                if start.x < end.x { 1 } else { -1 },
                if start.y < end.y { 1 } else { -1 },
            )
                .into(),
            error,
        }
    }
}

impl Iterator for LineAlgorithm {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.dest {
            return None;
        }
        let item = self.current.clone();
        let e2 = 2 * self.error;
        if e2 >= self.delta.y {
            if self.current.x == self.dest.x {
                return None;
            }
            self.error += self.delta.y;
            self.current.x += self.increment.x;
        }
        if e2 <= self.delta.x {
            if self.current.y == self.dest.y {
                return None;
            }
            self.error += self.delta.x;
            self.current.y += self.increment.y;
        }
        Some(item)
    }
}

pub struct VectorIter {
    line_iter: LineAlgorithm,
}

impl VectorIter {
    pub fn new(v: Vector) -> VectorIter {
        let mut line_iter = LineAlgorithm::new(
            (0, 0).into(),
            Point {
                x: v.x as i64,
                y: v.y as i64,
            },
        );
        line_iter.next();
        VectorIter { line_iter }
    }
}

impl Iterator for VectorIter {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.line_iter.next();
        let cur = self.line_iter.current;
        prev.map(|p| cur - p)
    }
}

pub struct SimpleLineIter {
    current: Option<Point>,
    dest: Point,
}

impl SimpleLineIter {
    pub fn new(start: Point, end: Point) -> SimpleLineIter {
        SimpleLineIter {
            current: Some(start),
            dest: end,
        }
    }
}

impl Iterator for SimpleLineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.as_mut() {
            let next = *current;
            let diff = self.dest - *current;
            let increment = diff / diff;
            println!("\t\t{:?}", diff);
            if increment.x.abs() > 0.0 {
                current.x += (increment.x / increment.x) as i64;
            }
            if increment.y.abs() > 0.0 {
                current.y += (increment.y / increment.y) as i64;
            }
            if diff.magnitude() == 0.0 {
                self.current = None;
            }
            Some(next)
        } else {
            None
        }
    }
}
