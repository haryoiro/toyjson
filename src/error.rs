use crate::token::{Location, Token, TokenType};

#[derive(Debug)]
pub enum JSONError {
    IOError(std::io::Error),
    StringError(String),
    LexcalError(String, Location),
}

impl From<std::io::Error> for JSONError {
    fn from(e: std::io::Error) -> Self {
        JSONError::IOError(e)
    }
}
