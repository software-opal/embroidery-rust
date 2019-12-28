use std::io::Read;
use std::io::Write;

use crate::collection::PatternCollection;
use crate::errors::{ReadResult, WriteResult};

pub trait CollectionFormat {
    fn name<'a>(&self) -> &'a str;
    fn extensions<'a, 'b>(&self) -> &'a [&'b str];
    fn reader(&self) -> Option<Box<dyn CollectionReader>>;
    fn writer(&self) -> Option<Box<dyn CollectionWriter>>;
}

pub trait CollectionReader {
    /// Returns true when the file is able to be loaded by this `CollectionReader`.
    /// Ideally this should inspect the file's magic number, or some metadata; this shouldn't load
    /// the entire file, nor should it perform a check that the contents is valid(unless of course
    /// that is the easiest way, for example when checking that a JSON document is actually a
    /// pattern).
    fn is_loadable(&self, item: &mut dyn Read) -> ReadResult<bool>;

    /// Read the pattern from the file and return it.
    fn read_pattern(&self, item: &mut dyn Read) -> ReadResult<PatternCollection>;
}

pub trait CollectionWriter {
    /// Write a PatternCollection to a file
    fn write_pattern(&self, pattern: &PatternCollection, writer: &mut dyn Write) -> WriteResult<()>;
}
