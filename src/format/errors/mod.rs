pub mod read;
pub mod std;
pub mod write;

pub use self::read::{ReadError, ReadResult};
pub use self::std::StdError;
pub use self::write::{WriteError, WriteResult};
use std::result;

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
