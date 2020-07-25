use std::num::{ParseFloatError, ParseIntError};
use std::error::Error;

#[derive(Clone, Debug)]
pub enum SpriosError {
    WorldParseError(String),
}

impl From<std::num::ParseFloatError> for SpriosError {

    fn from(e: ParseFloatError) -> Self {
        SpriosError::WorldParseError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for SpriosError {

    fn from(e: ParseIntError) -> Self {
        SpriosError::WorldParseError(e.to_string())
    }
}

impl From<std::io::Error> for SpriosError {

    fn from(e: std::io::Error) -> Self {
        SpriosError::WorldParseError(e.to_string())
    }
}
