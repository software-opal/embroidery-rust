use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

use embroidery_fmt_csv::CsvPatternFormat;
use embroidery_fmt_dst::DstPatternFormat;
use embroidery_fmt_hus::{HusPatternFormat, VipPatternFormat};
use embroidery_fmt_svg::SvgPatternFormat;
use embroidery_fmt_vp3::Vp3PatternFormat;

pub fn get_all() -> Vec<Box<dyn PatternFormat>> {
    vec![
        Box::new(CsvPatternFormat::default()),
        Box::new(DstPatternFormat::default()),
        Box::new(HusPatternFormat::default()),
        Box::new(SvgPatternFormat::default()),
        Box::new(VipPatternFormat::default()),
    ]
}
pub fn get_readers() -> Vec<Box<dyn PatternReader>> {
    get_all().into_iter().filter_map(|format| format.reader()).collect()
}

pub fn get_writers() -> Vec<Box<dyn PatternWriter>> {
    get_all().into_iter().filter_map(|format| format.writer()).collect()
}
