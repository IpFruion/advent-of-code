#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub mod errors;

/// produces an iterator of lines from a file
/// ignores an error when reading a line and returns early
pub fn safe_lines<S: AsRef<Path>>(filename: S) -> Result<impl Iterator<Item = String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines().map_while(|r| r.ok()))
}
