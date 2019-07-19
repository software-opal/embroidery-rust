use std::result;

use super::StdError;
use crate::stitch::Stitch;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Unable to write stitch {:?} at {:?}", stitch, idx)]
    UnsupportedStitch { stitch: Stitch, idx: Option<usize> },

    #[fail(display = "{}", _0)]
    Std(#[cause] StdError),
}

impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::Std(err.into())
    }
}

pub type Result<T> = result::Result<T, Error>;
