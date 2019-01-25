#[macro_use]
pub extern crate failure;
#[macro_use]
#[allow(unused_imports)]
pub extern crate log;

pub mod format;
pub mod geom;
pub mod pattern;

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};

    pub use crate::format::errors::{Error, ReadError, WriteError};
    pub use crate::pattern::{Color, ColorGroup, Pattern, PatternAttribute};
    pub use crate::pattern::{Stitch, StitchGroup, Thread};
}
