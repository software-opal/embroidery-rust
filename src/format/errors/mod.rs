pub use self::read::{ReadError, ReadResult};
pub use self::write::{WriteError, WriteResult};
use std::fmt;
use std::io;
use std::result;

pub mod read;
pub mod write;

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

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Read Error: {}", _0)]
    Read(#[cause] ReadError),

    #[fail(display = "Write Error: {}", _0)]
    Write(#[cause] WriteError),

    #[fail(display = "Other Error: {}", _0)]
    Standard(#[cause] StdError),
}

impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        Error::Read(err)
    }
}
impl From<WriteError> for Error {
    fn from(err: WriteError) -> Self {
        Error::Write(err.into())
    }
}
impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::Standard(err.into())
    }
}

pub type Result<T> = result::Result<T, Error>;
