
use std::io::Read;

use embroidery_lib::prelude::*;
use archivelib::do_compress;

use crate::header::VipHeader;
use crate::consts::VIP_DECODING_TABLE;

pub enum VipAttributes {
    Normal,
    Jump,
    ColorChange,
    LastStitch,
}

#[derive(Default)]
pub struct VipPatternLoader {}

impl PatternLoader for VipPatternLoader {
    fn is_loadable(&self, item: &mut Read) -> ReadResult<bool> {
        // Load the header
        // Check the last byte of the file? maybe
        return match VipHeader::build(item) {
            Err(ReadError::InvalidFormatError(_)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        };
    }

    fn read_pattern(&self, item: &mut Read) -> ReadResult<Pattern> {
        // Read the header
        let header = VipHeader::build(item)?;
        let colors = read_colors(&header, item)?;
        let attributes = read_attributes(&header, item)?;

        // let color_groups = read_stitches(&mut iter)?;
        // let (title, attributes) = extract_title(attributes);
        // return Ok(Pattern {
        //     name: title,
        //     attributes: attributes,
        //     color_groups: color_groups,
        // });
        panic!();
    }
}

fn read_colors(header: &VipHeader, item: &mut Read) -> ReadResult<Vec<Color>> {
    let len = header.color_len();
    assert!(len <= VIP_DECODING_TABLE.len());
    let mut values = vec![0; len];
    item.read_exact(&mut values)?;

    let mut prev = 0u8;
    let mut colors = vec![0; len];
    for (i, &v) in values.iter().enumerate() {
        let tmp = v ^ VIP_DECODING_TABLE[i];
        colors[i] = tmp ^ prev;
        prev = v;
    }

    Ok(        values
            .chunks_exact(4)
            .map(|colors| Color {
                red: colors[0],
                green: colors[1],
                blue: colors[2],
            })
            .collect()    )
}


fn read_attributes(header: &VipHeader, item &mut Read) -> ReadResult<Vec<VipAttribute>> {
    item.take(header.attribute_len())

    // Normal
    // Jump
    // ColorChange
    // LastStitch


}
