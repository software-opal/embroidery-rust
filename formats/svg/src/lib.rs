mod write;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

pub use self::write::SvgPatternWriter;

const NAME: &str = "svg";
const EXTENSIONS: [&str; 1] = ["svg"];

#[derive(Default)]
pub struct SvgPatternFormat {}

impl PatternFormat for SvgPatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        None
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        Some(Box::from(SvgPatternWriter::default()))
    }
}
