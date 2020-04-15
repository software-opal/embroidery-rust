// mod read;
mod write;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

// pub use self::read::CsvPatternReader;
pub use self::write::CsvPatternWriter;

const NAME: &str = "csv";
const EXTENSIONS: [&str; 1] = ["csv"];

#[derive(Default)]
pub struct CsvPatternFormat {}

impl PatternFormat for CsvPatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        None
        // Some(CsvPatternReader::default())
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        let writer = Box::from(CsvPatternWriter::default());
        Some(writer)
    }
}
