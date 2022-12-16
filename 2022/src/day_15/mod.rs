use std::ops::Range;

use advent_of_code::errors::Result;

use self::{
    point::Point,
    range::{Contains, RangeExt},
    sensor::Sensor,
};

mod point;
mod range;
mod sensor;

fn print_sensors(v: &Vec<Sensor>, max: isize) {
    for y in 0..=max {
        for x in 0..=max {
            let p = (x, y).into();
            let c = if v.iter().find(|s| s.is_inside(&p)).is_some() {
                '#'
            } else {
                '.'
            };
            print!("{}", c)
        }
        println!()
    }
}

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L, row: isize) -> Result<usize> {
    let mut sensors: Vec<Sensor> = lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|l| l.as_ref().parse())
        .collect::<Result<_>>()?;

    // sensors.sort_unstable_by(|a, b| {
    //     let a = row - a.pos().y;
    //     let b = row - b.pos().y;
    //     a.abs().cmp(&b.abs())
    // });
    sensors.retain(|s| {
        row <= s.pos().y + s.distance_to_beacon() && row >= s.pos().y - s.distance_to_beacon()
    });
    println!("{:?}", sensors);

    let min_x = sensors
        .iter()
        .flat_map(|s| [s.pos(), s.beacon()])
        .min_by(|a, b| a.x.cmp(&b.x))
        .ok_or("Couldn't find min x")?;

    let max_x = sensors
        .iter()
        .flat_map(|s| [s.pos(), s.beacon()])
        .max_by(|a, b| a.x.cmp(&b.x))
        .ok_or("Couldn't find max x")?;

    let max_dist = sensors
        .iter()
        .map(|s| s.distance_to_beacon())
        .max()
        .ok_or("Couldn't find max dist")?;

    let mut blocked = 0;
    //terribly inefficient
    for x in (min_x.x - max_dist)..(max_x.x + max_dist) {
        let p: Point = (x, row).into();
        if sensors.iter().find(|s| s.is_inside(&p)).is_some() {
            blocked += 1;
        }
    }
    Ok(blocked - 1)
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L, max: isize) -> Result<usize> {
    let mut sensors: Vec<Sensor> = lines
        .filter(|l| !l.as_ref().trim().is_empty())
        .map(|l| l.as_ref().parse())
        .collect::<Result<_>>()?;

    sensors.sort_unstable_by(|a, b| {
        let a_y = a.pos().y - a.distance_to_beacon();
        let b_y = b.pos().y - b.distance_to_beacon();
        a_y.cmp(&b_y)
    });
    // let sensors: VecDeque<Sensor> = sensors.into_iter().collect();
    for y in 0..max {
        let mut row: Vec<Range<isize>> = Vec::new();
        for s in sensors.iter() {
            if let Some(s) = s.x_range(y) {
                row.push(s);
            }
        }
        row.sort_unstable_by(|a, b| a.compare(b));
        // println!("{:?}", row);
        row = row.into_iter().fold(Vec::new(), |mut accum, s| {
            // println!("\t{:?}", s);
            let v = accum
                .iter_mut()
                .find_map(|r| r.clone().merge(s.clone()).map(|k| (r, k)));
            if let Some((r1, r2)) = v {
                *r1 = r2;
            } else {
                accum.push(s);
            }

            accum
        });
        let search = 0..max;
        // Looking for a row that doesn't cantain 0..max
        let is_blocked = row
            .iter()
            .any(|r| matches!(r.contains_range(&search), Some(Contains::Full)));
        if !is_blocked {
            let pos: Point = (row[0].end + 1, y).into();
            println!("{}", pos);
            return Ok(pos.x as usize * 4_000_000 + pos.y as usize);
        }
        // println!("{:?}", row);
    }

    // let blocked_points: HashSet<Point> = sensors.iter().flat_map(|s| s.blocked_points()).collect();
    // println!("Built {}", blocked_points.len());
    //
    // for y in 0..=max {
    //     for x in 0..=max {
    //         let p: Point = (x, y).into();
    //         if !blocked_points.contains(&p) {
    //             println!("{}", p);
    //             return Ok(p.x as usize * 4_000_000 + p.y as usize);
    //         }
    //     }
    // }

    // println!("{}", sensors[6]);
    // for p in sensors[6].blocked_points() {
    //     println!("{}", p);
    // }

    // new strategy is to condense the above structure into a list of ranges

    // let beacons = sensors.iter().fold(HashMap::new(), |mut accum, s| {
    //     let v = accum.entry(*s.beacon()).or_insert(Vec::new());
    //     v.push(*s.pos());
    //     accum
    // });

    // for y in 0..=max {
    //     for x in 0..=max {
    //         let p: Point = (x, y).into();
    //
    //         let is_blocked = sensors.iter().any(|s| {
    //             let dist = s.pos().manhatten_distance_to(&p);
    //             dist <= s.distance_to_beacon()
    //         });
    //
    //         if !is_blocked {
    //             println!("{}", p);
    //             return Ok(p.x as usize * 4_000_000 + p.y as usize);
    //         }
    //     }
    // }
    Err("Beacon Not Found".into())
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;
    #[test]
    fn page_example_1() {
        let actual = solution_pt1(PAGE_EXAMPLE.lines(), 10).unwrap();
        assert_eq!(actual, 26)
    }

    #[test]
    fn page_example_2() {
        let actual = solution_pt2(PAGE_EXAMPLE.lines(), 20).unwrap();
        assert_eq!(actual, 56_000_011)
    }
}
