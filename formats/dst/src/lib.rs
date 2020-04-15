mod read;
mod stitch_info;
mod write;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

pub use self::read::DstPatternReader;
pub use self::write::DstPatternWriter;

const NAME: &str = "dst";
const EXTENSIONS: [&str; 1] = ["dst"];

#[derive(Default)]
pub struct DstPatternFormat {}

impl PatternFormat for DstPatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(DstPatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        Some(Box::from(DstPatternWriter::default()))
    }
}
