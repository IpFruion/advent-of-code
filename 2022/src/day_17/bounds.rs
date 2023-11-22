use std::{cmp::Ordering, fmt::Display};

use super::{piece::Piece, point::Point, shape::Shape};

/// Constructs a bounding box with the points stored as the bottom left and top right
#[derive(Debug, Clone, Copy)]
pub struct Bounds(Point, Point);

impl Default for Bounds {
    fn default() -> Self {
        Bounds((0, 0).into(), (0, 0).into())
    }
}

impl Bounds {
    pub const fn zero_based(x: isize, y: isize) -> Self {
        Bounds(Point { x: 0, y: 0 }, Point { x, y })
    }

    pub fn contains(&self, p: &Point) -> bool {
        !(self.0.x > p.x || self.0.y > p.y || self.1.x <= p.x || self.1.y <= p.y)
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        self.0
    }

    #[inline]
    pub fn top_right(&self) -> Point {
        self.1
    }

    #[inline]
    pub fn bottom_right(&self) -> Point {
        (self.1.x, self.0.y).into()
    }

    pub fn shift(&mut self, p: &Point) {
        self.0.x += p.x;
        self.0.y += p.y;
        self.1.x += p.x;
        self.1.y += p.y;
    }

    pub fn shape(&self, piece: &Piece) -> Shape {
        let mut shape: Shape = piece.points().iter().copied().collect();
        shape.shift(&self.0);
        shape
    }
}

impl<'a> FromIterator<&'a Point> for Option<Bounds> {
    fn from_iter<T: IntoIterator<Item = &'a Point>>(iter: T) -> Self {
        let (bottom_left, top_right) =
            iter.into_iter()
                .fold((None::<Point>, None::<Point>), |accum, p| {
                    let (mut min, mut max) = accum;
                    if let Some(min) = min.as_mut() {
                        if min.x.cmp(&p.x) == Ordering::Greater {
                            min.x = p.x;
                        }
                        if min.y.cmp(&p.y) == Ordering::Greater {
                            min.y = p.y;
                        }
                    } else {
                        min = Some(p.clone())
                    }

                    if let Some(max) = max.as_mut() {
                        if max.x.cmp(&p.x) == Ordering::Less {
                            max.x = p.x;
                        }
                        if max.y.cmp(&p.y) == Ordering::Less {
                            max.y = p.y;
                        }
                    } else {
                        max = Some(p.clone())
                    }

                    (min, max)
                });
        Some(Bounds(
            bottom_left?,
            //not inclusive
            top_right.map(|p| (p.x + 1, p.y + 1).into())?,
        ))
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
