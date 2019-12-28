#[macro_use]
pub extern crate failure;
#[macro_use]
#[allow(unused_imports)]
pub extern crate log;

mod collection;
mod colors;
mod pattern;
mod stitch_util;
mod stitch;
mod str_util;
mod byte_utils;

pub mod errors;
pub mod format;

pub use crate::colors::Color;
pub use crate::collection::PatternCollection;
pub use crate::errors::{Error, ReadError, WriteError};
pub use crate::pattern::{Pattern, PatternAttribute};
pub use crate::stitch::{ColorGroup, StitchGroup, Stitch, Thread};

pub mod utils {
    pub use crate::stitch_util::{StitchInfo, build_stitch_list};
    pub use crate::str_util::{c_trim, char_truncate};
    pub use crate::byte_utils::ReadByteIterator;
}

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};

    pub use crate::colors::Color;
    pub use crate::errors::{Error, ReadError, WriteError};
    pub use crate::pattern::{Pattern, PatternAttribute};
    pub use crate::stitch::{ColorGroup, Stitch, StitchGroup, Thread};
    pub use crate::collection::PatternCollection;
}
