use embroidery_lib::format::traits::{PatternLoader, PatternWriter};

use embroidery_fmt_csv::CsvPatternWriter;
use embroidery_fmt_dst::{DstPatternLoader, DstPatternWriter};
use embroidery_fmt_hus::HusVipPatternLoader;
use embroidery_fmt_svg::SvgPatternWriter;

type BoxedPatternLoader = Box<dyn PatternLoader>;
type BoxedPatternWriter = Box<dyn PatternWriter>;

pub fn get_all() -> Vec<(Option<BoxedPatternLoader>, Option<(String, BoxedPatternWriter)>)> {
    vec![
        (
            Some(Box::new(DstPatternLoader::default())),
            Some(("dst".to_string(), Box::new(DstPatternWriter::default()))),
        ),
        (Some(Box::new(HusVipPatternLoader::default())), None),
        (None, Some(("svg".to_string(), Box::new(SvgPatternWriter::default())))),
        (None, Some(("csv".to_string(), Box::new(CsvPatternWriter::default())))),
    ]
}

pub fn get_loaders() -> Vec<std::boxed::Box<dyn PatternLoader>> {
    get_all().into_iter().filter_map(|(loader, _)| loader).collect()
}
pub fn get_writers() -> Vec<(String, Box<dyn PatternWriter>)> {
    get_all().into_iter().filter_map(|(_, writer)| writer).collect()
}
