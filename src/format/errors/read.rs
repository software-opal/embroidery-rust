use std::result;

use super::std::StdError;

#[derive(Fail, Debug)]
pub enum ReadError {
    #[fail(display = "Invalid format: {}", _0)]
    InvalidFormatError(String),
    #[fail(display = "{}", _0)]
    Std(#[cause] StdError),
}

impl <T: Into<StdError>> From<T> for ReadError {
    fn from(err: T) -> Self {
        ReadError::Std(err.into())
    }
}

pub type ReadResult<T> = result::Result<T, ReadError>;
