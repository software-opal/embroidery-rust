use std::io::Read;
use std::io::Write;

use crate::format::errors::{ReadResult, WriteResult};
use crate::pattern::Pattern;

pub trait PatternLoader {
    /// Returns true when the file is able to be loaded by this `PatternLoader`.
    /// Ideally this should inspect the file's magic number, or some metadata; this shouldn't load
    /// the entire file, nor should it perform a check that the contents is valid(unless of course
    /// that is the easiest way, for example when checking that a JSON document is actually a
    /// pattern).
    fn is_loadable(&self, item: &mut dyn Read) -> ReadResult<bool>;

    /// Read the pattern from the file and return it.
    fn read_pattern(&self, item: &mut dyn Read) -> ReadResult<Pattern>;
}

pub trait PatternWriter {
    /// Write a pattern to a file
    fn write_pattern(&self, pattern: &Pattern, writer: &mut dyn Write) -> WriteResult<()>;
}
