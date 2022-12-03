use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidParseError(String),
    InvalidStruct(String),
}

impl From<ParseIntError> for Error {
    fn from(i: ParseIntError) -> Self {
        Error::InvalidParseError(format!("{:?}", i))
    }
}
