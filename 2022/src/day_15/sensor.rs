use std::{fmt::Display, ops::Range, str::FromStr};

use advent_of_code::errors::Error;

use super::point::Point;

#[derive(Debug)]
pub struct Sensor {
    pos: Point,
    beacon: Point,
}

impl Sensor {
    pub fn distance_to_beacon(&self) -> isize {
        self.pos.manhatten_distance_to(&self.beacon)
    }
    pub fn is_inside(&self, p: &Point) -> bool {
        let dist = self.pos.manhatten_distance_to(p);
        dist <= self.distance_to_beacon()
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn beacon(&self) -> &Point {
        &self.beacon
    }
    pub fn blocked_points(&self) -> impl Iterator<Item = Point> + '_ {
        let dist = self.pos.manhatten_distance_to(&self.beacon);
        ((self.pos.y - dist)..=(self.pos.y + dist)).flat_map(move |y| {
            let dist_c = dist;
            ((self.pos.x - dist)..=(self.pos.x + dist))
                .map(move |x| (x, y).into())
                .filter(move |p: &Point| p.manhatten_distance_to(&self.pos) <= dist_c)
        })
    }

    pub fn x_range(&self, y: isize) -> Option<Range<isize>> {
        let dist = self.distance_to_beacon();
        let y_diff = (y - self.pos.y).abs();
        if dist < y_diff {
            return None;
        }

        let k = (y_diff - dist).abs();

        //not sure about the 1 here
        Some((self.pos.x - k)..(self.pos.x + k))
    }

    pub fn space(&self) -> [Point; 4] {
        self.pos.manhatten_box(&self.beacon)
    }
}

impl FromStr for Sensor {
    type Err = Error;

    /// example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut at_splits = s.trim().split("at").skip(1);

        let pos = at_splits
            .next()
            .ok_or(Error::InvalidParseError("No Sensor location".to_owned()))?
            .split(":")
            .next()
            .ok_or(Error::InvalidParseError("No Sensor location".to_owned()))?
            .parse()?;

        let beacon = at_splits
            .next()
            .ok_or(Error::InvalidParseError("No Beacon location".to_owned()))?
            .parse()?;
        Ok(Sensor { pos, beacon })
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.pos, self.beacon)
    }
}
