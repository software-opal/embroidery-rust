use std::result;

use super::StdError;
use crate::stitch::Stitch;

#[derive(Fail, Debug)]
pub enum WriteError {
    #[fail(display = "Unable to write stitch {:?} at {:?}", stitch, idx)]
    UnsupportedStitch { stitch: Stitch, idx: Option<usize> },

    #[fail(display = "{}", _0)]
    Std(#[cause] StdError),
}

impl<T: Into<StdError>> From<T> for WriteError {
    fn from(err: T) -> Self {
        WriteError::Std(err.into())
    }
}

pub type WriteResult<T> = result::Result<T, WriteError>;
