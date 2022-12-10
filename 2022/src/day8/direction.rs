pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn iter<'a>(
        &'a self,
        start_point: (usize, usize),
        max: (usize, usize),
    ) -> DirectionIterator<'a> {
        DirectionIterator {
            direction: self,
            cur_point: start_point,
            max,
        }
    }
}

pub struct DirectionIterator<'a> {
    direction: &'a Direction,
    cur_point: (usize, usize),
    max: (usize, usize),
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next_point = match self.direction {
            Direction::Up => (self.cur_point.0.checked_sub(1)?, self.cur_point.1),
            Direction::Right => (
                self.cur_point.0,
                max_or_option(self.cur_point.1 + 1, self.max.1)?,
            ),
            Direction::Down => (
                max_or_option(self.cur_point.0 + 1, self.max.0)?,
                self.cur_point.1,
            ),
            Direction::Left => (self.cur_point.0, self.cur_point.1.checked_sub(1)?),
        };
        self.cur_point = next_point;
        Some(next_point)
    }
}

fn max_or_option(v: usize, max: usize) -> Option<usize> {
    if v < max {
        return Some(v);
    }
    None
}
