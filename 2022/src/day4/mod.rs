use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Bound, RangeBounds, RangeInclusive},
};

use advent_of_code::errors::{Error, Result};

fn into_range(line: &str) -> Result<RangeInclusive<u64>> {
    let (lower_bound, upper_bound) = line
        .split_once('-')
        .ok_or(Error::InvalidParseError("no dash in input".to_owned()))?;

    let lower_bound = u64::from_str_radix(lower_bound, 10)?;
    let upper_bound = u64::from_str_radix(upper_bound, 10)?;
    Ok(lower_bound..=upper_bound)
}

pub trait RangeContains<T: ?Sized> {
    fn fully_contains<R: RangeBounds<T>>(&self, r: &R) -> bool;
    fn partially_contains<R: RangeBounds<T>>(&self, r: &R) -> bool;
}

pub trait CompareBounds<T: PartialOrd<T>> {
    fn cmp_bounds(&self, b: Bound<T>) -> Option<Ordering>;
}

impl<T: PartialOrd<T>> CompareBounds<T> for Bound<T> {
    fn cmp_bounds(&self, b: Bound<T>) -> Option<Ordering> {
        match (self, b) {
            (Bound::Unbounded, Bound::Unbounded) => Some(Ordering::Equal),
            (Bound::Included(lower), Bound::Included(upper))
            | (Bound::Excluded(lower), Bound::Included(upper))
            | (Bound::Included(lower), Bound::Excluded(upper))
            | (Bound::Excluded(lower), Bound::Excluded(upper)) => lower.partial_cmp(&upper),
            _ => None,
        }
    }
}

impl<T: PartialOrd<T> + Debug> RangeContains<T> for RangeInclusive<T> {
    fn fully_contains<R: RangeBounds<T>>(&self, r: &R) -> bool {
        let start = self.start_bound().cmp_bounds(r.start_bound());
        let end = self.end_bound().cmp_bounds(r.end_bound());

        start
            .map(|start| matches!(start, Ordering::Greater | Ordering::Equal))
            .unwrap_or(false)
            && end
                .map(|end| matches!(end, Ordering::Less | Ordering::Equal))
                .unwrap_or(false)
    }

    fn partially_contains<R: RangeBounds<T>>(&self, r: &R) -> bool {
        if self.fully_contains(r) {
            return true;
        }
        // ___|____| self
        // |____|___ r
        let start = self.start_bound().cmp_bounds(r.start_bound());
        let end = self.start_bound().cmp_bounds(r.end_bound());
        if let (Some(start), Some(end)) = (start, end) {
            if matches!(start, Ordering::Greater | Ordering::Equal)
                && matches!(end, Ordering::Less | Ordering::Equal)
            {
                return true;
            }
        }

        // |____|___ self
        // ___|____| r

        let start = self.end_bound().cmp_bounds(r.start_bound());
        let end = self.end_bound().cmp_bounds(r.end_bound());
        if let (Some(start), Some(end)) = (start, end) {
            if matches!(start, Ordering::Greater | Ordering::Equal)
                && matches!(end, Ordering::Less | Ordering::Equal)
            {
                return true;
            }
        }

        false
    }
}

pub fn solution_pt1<S: AsRef<str>, I: Iterator<Item = S>>(lines: I) -> Result<u64> {
    lines
        .map(|line| {
            let (first_pair, second_pair) = line
                .as_ref()
                .split_once(',')
                .ok_or(Error::InvalidParseError("no pairing".to_owned()))?;
            let first_pair = into_range(first_pair)?;
            let second_pair = into_range(second_pair)?;
            let first_contains_second = first_pair.fully_contains(&second_pair);
            let second_contains_first = second_pair.fully_contains(&first_pair);

            // println!(
            //     "{:?}, {:?}, {}, {}",
            //     first_pair, second_pair, first_contains_second, second_contains_first,
            // );

            if first_contains_second || second_contains_first {
                return Ok(1);
            }
            Ok(0)
        })
        .sum()
}

pub fn solution_pt2<S: AsRef<str>, I: Iterator<Item = S>>(lines: I) -> Result<u64> {
    lines
        .map(|line| {
            let (first_pair, second_pair) = line
                .as_ref()
                .split_once(',')
                .ok_or(Error::InvalidParseError("no pairing".to_owned()))?;
            let first_pair = into_range(first_pair)?;
            let second_pair = into_range(second_pair)?;
            let first_contains_second = first_pair.partially_contains(&second_pair);
            let second_contains_first = second_pair.partially_contains(&first_pair);

            if first_contains_second || second_contains_first {
                return Ok(1);
            }
            Ok(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use advent_of_code::safe_lines;

    use super::{solution_pt1, solution_pt2};

    #[test]
    fn page_example_pt_1() {
        let lines = safe_lines("input/page_example_4.txt").unwrap();
        let res = solution_pt1(lines).unwrap();

        assert_eq!(res, 2);
    }

    #[test]
    fn page_example_pt_2() {
        let lines = safe_lines("input/page_example_4.txt").unwrap();
        let res = solution_pt2(lines).unwrap();

        assert_eq!(res, 4);
    }
}
