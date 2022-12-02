use std::{cmp::Reverse, collections::BinaryHeap};

use crate::errors::Result;

fn attempt_push_max(arr: &mut [usize], val: usize) {
    let mut set_max = false;
    let mut shift_amount = 0;
    for i in 0..arr.len() {
        if shift_amount > arr[i] {
            let tmp = arr[i];
            arr[i] = shift_amount;
            shift_amount = tmp;
        }
        if !set_max && val > arr[i] {
            shift_amount = arr[i];
            arr[i] = val;
            set_max = true;
        }
    }
}

/// Solution however the time complexity could be probably improved. If E is large then the runtime
/// is O(n * E) where n is the length of the input lines
pub fn solution<const E: usize, S: AsRef<str>, I: Iterator<Item = S>>(lines: I) -> Result<usize> {
    lines
        .fold(Ok(([0; E], 0)), |accum, line| {
            let (mut max_calories, current_calories) = accum?;
            let line = line.as_ref().trim();
            if line.is_empty() {
                attempt_push_max(&mut max_calories, current_calories);
                return Ok((max_calories, 0));
            }
            let calories = usize::from_str_radix(line, 10)?;
            Ok((max_calories, current_calories + calories))
        })
        .map(|(mut max_calores, last_item)| {
            if last_item > 0 {
                attempt_push_max(&mut max_calores, last_item);
            }
            max_calores.iter().sum()
        })
}

/// From understanding what the max was actually doing is a Binary Heap. Rust as a binary heap impl
/// that I am going to use!
pub fn solution_improved<S: AsRef<str>, I: Iterator<Item = S>>(
    elves: usize,
    lines: I,
) -> Result<usize> {
    let mut heap = BinaryHeap::new();
    lines
        .fold(Ok((&mut heap, 0)), |accum, line| {
            let (heap, current_calories) = accum?;
            let line = line.as_ref().trim();
            if line.is_empty() {
                heap.push(Reverse(current_calories));
                if heap.len() > elves {
                    heap.pop();
                }
                return Ok((heap, 0));
            }
            let calories = usize::from_str_radix(line, 10)?;
            Ok((heap, current_calories + calories))
        })
        .map(|(heap, last_item)| {
            if last_item > 0 {
                heap.push(Reverse(last_item));
                if heap.len() > elves {
                    heap.pop();
                }
            }
            let mut sum = 0;
            while let Some(v) = heap.pop() {
                sum += v.0;
            }
            sum
        })
}

#[cfg(test)]
mod tests {
    use crate::safe_lines;

    use super::*;

    #[test]
    fn pt_1_page_example() {
        let calories = safe_lines("input/page_example_1.txt").unwrap();
        let top_calories = solution::<1, _, _>(calories).unwrap();

        assert_eq!(top_calories, 24_000)
    }

    #[test]
    fn pt_2_page_example() {
        let calories = safe_lines("input/page_example_1.txt").unwrap();
        let top_calories = solution::<3, _, _>(calories).unwrap();

        assert_eq!(top_calories, 45_000)
    }
}
