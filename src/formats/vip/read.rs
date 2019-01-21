use super::header::VipHeader;
use crate::format::errors::{ReadError, ReadResult};
use crate::format::traits::PatternLoader;
use crate::pattern::colors::Color;
use crate::pattern::pattern::Pattern;
use std::io::Read;

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

// fn read_colors(header: &VipHeader, item: &mut Read) -> ReadResult<Vec<Color>> {
//     for(i = 0; i < header.numberOfColors*4; ++i)
//     {
//         unsigned char inputByte = binaryReadByte(file);
//         unsigned char tmpByte = (unsigned char) (inputByte ^ vipDecodingTable[i]);
//         decodedColors[i] = (unsigned char) (tmpByte ^ prevByte);
//         prevByte = inputByte;
//     }
//     for(i = 0; i < header.numberOfColors; i++)
//     {
//         EmbThread thread;
//         int startIndex = i << 2;
//         thread.color.r = decodedColors[startIndex];
//         thread.color.g = decodedColors[startIndex + 1];
//         thread.color.b = decodedColors[startIndex + 2];
//         /* printf("%d\n", decodedColors[startIndex + 3]); */
//         embPattern_addThread(pattern, thread);
//     }
//
// }
