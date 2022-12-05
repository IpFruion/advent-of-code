use std::cmp::Ordering;
use std::ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use super::bounds::CompareBounds;

#[derive(Debug)]
pub enum Contains {
    Fully,
    Partially,
}

pub trait Container<T> {
    fn contains(&self, t: T) -> Option<Contains>;
}

#[allow(unused_macros)]
macro_rules! range_container {
    ($struct_name:ty) => {
        impl<T: PartialOrd<T>, R: RangeBounds<T>> Container<&R> for $struct_name {
            fn contains(&self, r: &R) -> Option<Contains> {
                let start_start = self.start_bound().cmp_bounds(r.start_bound())?;
                let end_end = self.end_bound().cmp_bounds(r.end_bound())?;

                // __|___|__ self
                // ___|_|___ r
                if matches!(start_start, Ordering::Less | Ordering::Equal)
                    && matches!(end_end, Ordering::Greater | Ordering::Equal)
                {
                    return Some(Contains::Fully);
                }

                let start_end = self.start_bound().cmp_bounds(r.end_bound())?;

                // ___|____| self
                // |____|___ r
                if matches!(start_start, Ordering::Greater | Ordering::Equal)
                    && matches!(start_end, Ordering::Less | Ordering::Equal)
                {
                    return Some(Contains::Partially);
                }

                // |____|___ self
                // ___|____| r
                let end_start = self.end_bound().cmp_bounds(r.start_bound())?;
                if matches!(end_end, Ordering::Less | Ordering::Equal)
                    && matches!(end_start, Ordering::Greater | Ordering::Equal)
                {
                    return Some(Contains::Partially);
                }

                None
            }
        }
    };
}

range_container!(Range<T>);
range_container!(RangeInclusive<T>);
range_container!(RangeTo<T>);
range_container!(RangeToInclusive<T>);
range_container!(RangeFrom<T>);
