extern crate embroidery_rust;

use embroidery_rust::formats::traits::PatternLoader;
use embroidery_rust::formats::traits::PatternWriter;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use embroidery_rust::formats::dst::DstPatternLoader;
use embroidery_rust::formats::svg::SvgPatternWriter;

static dstFile: &str = "tests/dst/test_data/OSHLogo.dst";
static svgFile: &str = "OSHLogo.svg";

fn main() {
    let dst = DstPatternLoader {};
    let svg = SvgPatternWriter {};
    let mut reader = BufReader::new(File::open(dstFile).expect("file not found"));
    let pattern = dst.read_pattern(&mut reader).expect("");

    let mut writer = BufWriter::new(File::create(svgFile).expect("file not found"));
    svg.write_pattern(&pattern, &mut writer).expect("");
}
