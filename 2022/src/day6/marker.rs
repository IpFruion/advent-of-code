use std::collections::{HashMap, HashSet, VecDeque};

pub struct Marker {
    data: VecDeque<char>,
    map: HashMap<char, usize>,
    size: usize,
    count: usize,
}

impl Marker {
    pub fn new(size: usize) -> Self {
        Marker {
            data: VecDeque::new(),
            map: HashMap::new(),
            size,
            count: 0,
        }
    }

    fn push_insert(&mut self, c: char) {
        self.data.push_back(c);
        if let Some(count) = self.map.get_mut(&c) {
            *count += 1;
        } else {
            self.map.insert(c, 1);
        }
        self.count += 1;
    }

    /// Pushes the character into the marker. Returns `Some(true)` for when there is a duplicate. `None`
    /// for when the marker hasn't been filled yet. `Some(false)` for when it is filled and there
    /// isn't a duplicate
    ///
    /// For runtime this method on each character costs: `O(hash(c))` since the operations done are
    /// `O(1)` for insert into linked list, removal from linked list, grabbing from hashmap,
    /// removing from hashmap, inserting into hashmap.
    pub fn push(&mut self, c: char) -> Option<bool> {
        // println!(
        //     "c = {} --> {:?} : {:?} : {}",
        //     c, self.data, self.map, self.count
        // );
        if self.data.len() < self.size {
            self.push_insert(c);
            return None;
        }
        if let Some(old) = self.data.pop_front() {
            let mut should_remove = false;
            if let Some(count) = self.map.get_mut(&old) {
                if *count - 1 == 0 {
                    should_remove = true;
                }
                *count -= 1;
                self.count -= 1;
            }
            if should_remove {
                self.map.remove(&old);
            }
        }
        self.push_insert(c);
        Some(self.map.len() == self.count)
    }
}
