use std::io::Read;

use embroidery_lib::format::PatternReader;
use embroidery_lib::prelude::*;

// use crate::colors::read_threads;
use crate::header::PatternHeader;

#[derive(Default)]
pub struct JefPatternReader {}

impl PatternReader for JefPatternReader {
    fn is_loadable(&self, item: &mut dyn Read) -> Result<bool, ReadError> {
        // Load the header
        // Check the last byte of the file? maybe
        return match PatternHeader::build(item) {
            Err(ReadError::InvalidFormat(_, _)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        };
    }

    fn read_pattern(&self, item: &mut dyn Read) -> Result<Pattern, ReadError> {
        // Read the header
        let _header = PatternHeader::build(item)?;
        unimplemented!();
        // let threads = read_threads(&header, item)?;
        // let attributes = read_attributes(&header, item)?;
        // let x_coords = read_x_coords(&header, item)?;
        // let y_coords = read_y_coords(&header, item)?;
        // if attributes.len() != x_coords.len() || attributes.len() != y_coords.len() {
        //     return Err(ReadError::invalid_format(format!(
        //         "Different numbers of attributes({}), x coordinates({}) and y coordinates({})",
        //         attributes.len(),
        //         x_coords.len(),
        //         y_coords.len()
        //     )));
        // }
        //
        // // let color_groups = read_stitches(&mut iter)?;
        // // let (title, attributes) = extract_title(attributes);
        // Ok(Pattern {
        //     name: "".to_string(),
        //     attributes: vec![],
        //     color_groups: convert_stitches(threads, attributes, x_coords, y_coords),
        // })
    }
}
