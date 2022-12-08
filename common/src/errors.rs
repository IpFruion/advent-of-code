use std::{
    cell::{BorrowError, BorrowMutError},
    num::ParseIntError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidParseError(String),
    InvalidStruct(String),
    BorrowError(String),
    RawError(String),
}

impl From<ParseIntError> for Error {
    fn from(i: ParseIntError) -> Self {
        Error::InvalidParseError(format!("{:?}", i))
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::RawError(s.to_string())
    }
}

impl From<BorrowError> for Error {
    fn from(e: BorrowError) -> Self {
        Error::BorrowError(format!("{:?}", e))
    }
}

impl From<BorrowMutError> for Error {
    fn from(e: BorrowMutError) -> Self {
        Error::BorrowError(format!("{:?}", e))
    }
}
