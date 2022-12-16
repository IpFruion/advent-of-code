use std::{cmp::Ordering, ops::Range};

pub enum Contains {
    Full,
    Partial,
}

pub trait RangeExt<T> {
    fn merge(self, r: Range<T>) -> Option<Range<T>>;
    fn compare(&self, r: &Range<T>) -> Ordering;
    fn contains_range(&self, r: &Range<T>) -> Option<Contains>;
}

impl<T: Ord> RangeExt<T> for Range<T> {
    fn merge(self, r: Range<T>) -> Option<Range<T>> {
        let start_end = self.start.cmp(&r.end);
        let end_start = self.end.cmp(&r.start);
        match (start_end, end_start) {
            (Ordering::Less, Ordering::Less) | (Ordering::Greater, Ordering::Greater) => {
                return None
            }
            _ => {}
        };
        let starts = self.start.cmp(&r.start);
        let ends = self.end.cmp(&r.end);
        match (starts, ends) {
            (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                Some(self.start..self.end)
            }
            (Ordering::Greater | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                Some(r.start..r.end)
            }
            (Ordering::Less, Ordering::Less) => Some(self.start..r.end),
            (Ordering::Greater, Ordering::Greater) => Some(r.start..self.end),
        }
    }

    fn compare(&self, r: &Range<T>) -> Ordering {
        self.start.cmp(&r.start).then_with(|| self.end.cmp(&r.end))
    }

    fn contains_range(&self, r: &Range<T>) -> Option<Contains> {
        let start_end = self.start.cmp(&r.end);
        let end_start = self.end.cmp(&r.start);
        match (start_end, end_start) {
            (Ordering::Less, Ordering::Less) | (Ordering::Greater, Ordering::Greater) => {
                return None
            }
            _ => {}
        };
        let starts = self.start.cmp(&r.start);
        let ends = self.end.cmp(&r.end);
        match (starts, ends) {
            (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                Some(Contains::Full)
            }
            (Ordering::Greater | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                Some(Contains::Partial)
            }
            (Ordering::Less, Ordering::Less) => Some(Contains::Partial),
            (Ordering::Greater, Ordering::Greater) => Some(Contains::Partial),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RangeExt;

    #[test]
    fn extended_bounds() {
        assert_eq!((0..4).merge(1..3).unwrap(), 0..4);
        assert_eq!((1..3).merge(0..4).unwrap(), 0..4);
        assert_eq!((-2..3).merge(0..4).unwrap(), -2..4);
        assert_eq!((1..5).merge(0..4).unwrap(), 0..5);
    }

    #[test]
    fn outside_of_bounds() {
        assert_eq!((0..2).merge(3..5), None);
        assert_eq!((3..5).merge(0..2), None);
    }

    #[test]
    fn edge_cases() {
        assert_eq!((0..2).merge(2..5).unwrap(), 0..5);
        assert_eq!((2..5).merge(0..2).unwrap(), 0..5);
    }
}
