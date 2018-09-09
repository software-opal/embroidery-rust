#![feature(crate_in_paths)]

extern crate euclid;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate palette;
extern crate svgtypes;
extern crate unicode_segmentation;

pub mod errors;
pub mod format;
pub mod geom;
pub mod pattern;
mod utils;

pub mod formats;
