use std::fmt::Display;

use super::Direction;
use advent_of_code::errors::{Error, Result};

pub struct Grid {
    grid: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_lines<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<Self> {
        let grid: Vec<Vec<u8>> = lines
            .map(|line| {
                line.as_ref()
                    .trim()
                    .chars()
                    .map(|c| {
                        u8::try_from((c as u32).checked_sub(48).ok_or("Invalid grid character")?)
                            .map_err(|e| Error::InvalidParseError(format!("{:?}", e)))
                    })
                    .collect::<Result<Vec<u8>>>()
            })
            .filter(|row| row.as_ref().map(|r| !r.is_empty()).unwrap_or(true))
            .collect::<Result<_>>()?;
        let height = grid.len();
        let width = grid
            .get(0)
            .map(|r| r.len())
            .ok_or("Grid must have at least one row")?;

        Ok(Grid {
            grid,
            height,
            width,
        })
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    /// Doesn't do any bounds checking because I am too lazy today ;)
    pub fn is_visible(&self, start: (usize, usize), direction: Direction) -> bool {
        let height = self.grid[start.0][start.1];

        for (r, c) in direction.iter(start, (self.height, self.width)) {
            if self.grid[r][c] >= height {
                return false;
            }
        }
        true
    }

    pub fn viewing_distance(&self, start: (usize, usize), direction: Direction) -> usize {
        let height = self.grid[start.0][start.1];
        let mut dist = 0;
        for (r, c) in direction.iter(start, (self.height, self.width)) {
            dist += 1;
            if self.grid[r][c] >= height {
                break;
            }
        }
        dist
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

