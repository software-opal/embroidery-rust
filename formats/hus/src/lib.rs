mod colors;
mod header;
mod read;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

pub use read::HusVipPatternReader;

const NAME: &'static str = "hus/vip";
const EXTENSIONS: [&'static str; 2] = ["hus", "vip"];

#[derive(Default)]
pub struct HusVipPatternFormat {}

impl PatternFormat for HusVipPatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(HusVipPatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        None
    }
}
