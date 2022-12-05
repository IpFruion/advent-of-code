use std::{cmp::Ordering, ops::Bound};

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
