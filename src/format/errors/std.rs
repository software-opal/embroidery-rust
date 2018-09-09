use std::fmt;
use std::io;

#[derive(Debug, Fail)]
pub enum StdError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Fmt(#[cause] fmt::Error),
}

impl From<io::Error> for StdError {
    fn from(err: io::Error) -> Self {
        StdError::Io(err)
    }
}

impl From<fmt::Error> for StdError {
    fn from(err: fmt::Error) -> Self {
        StdError::Fmt(err)
    }
}
