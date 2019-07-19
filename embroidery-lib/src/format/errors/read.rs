use std::result;

use super::StdError;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid format: {}", _0)]
    InvalidFormat(String),
    #[fail(display = "{}", _0)]
    Std(#[cause] StdError),
}

impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::Std(err.into())
    }
}

pub type Result<T> = result::Result<T, Error>;
