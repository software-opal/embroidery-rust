mod colors;
mod header;
mod hoops;
mod read;

pub use read::JefPatternReader;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

const NAME: &'static str = "jef";
const EXTENSIONS: [&'static str; 1] = ["jef"];

#[derive(Default)]
pub struct JefPatternFormat {}

impl PatternFormat for JefPatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(JefPatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        None
    }
}
