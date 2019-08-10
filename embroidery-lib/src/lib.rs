#[macro_use]
pub extern crate failure;
#[macro_use]
#[allow(unused_imports)]
pub extern crate log;

pub mod colors;
pub mod format;
pub mod geom;
pub mod pattern;
pub mod stitch;
pub mod util;

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};

    pub use crate::colors::Color;
    pub use crate::format::errors::{Error, ReadError, WriteError};
    pub use crate::pattern::{Pattern, PatternAttribute};
    pub use crate::stitch::{ColorGroup, Stitch, StitchGroup, Thread};
}
