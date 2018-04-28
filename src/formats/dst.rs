use std::io::Read;
use std::iter::FromIterator;

use formats::errors::{Error, ErrorKind, Result};
use formats::traits::PatternLoader;
use formats::utils::read_to_iter;
use pattern::pattern::{Pattern, PatternAttribute};

pub struct DstPatternLoader {}

impl PatternLoader for DstPatternLoader {
    fn is_loadable(&self, item: &mut Read) -> Result<bool> {
        // Load the header
        // Check the last byte of the file? maybe
        let mut iter = read_to_iter(item);
        return match read_dst_header(&mut iter) {
            Err(Error(ErrorKind::InvalidFormatError(_), _)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        };
    }

    fn read_pattern(&self, item: &mut Read) -> Result<Pattern> {
        // Read the header
        let mut iter = read_to_iter(item);
        read_dst_header(&mut iter)?;
        // return Err("SSSSS"::into());
        return Ok(Pattern {
            name: "dave".to_owned(),
            attributes: vec![],
            stitch_groups: vec![],
        });
    }
}

fn read_dst_header(item: &mut Iterator<Item = u8>) -> Result<Vec<PatternAttribute>> {
    let mut attrs: Vec<PatternAttribute> = Vec::new();
    let mut header_iter = item.take(512);

    loop {
        match read_header_item(&mut header_iter)? {
            Some(header) => attrs.push(header),
            None => break,
        }
    }
    return Ok(attrs);
}

fn read_header_item(mut header_iter: &mut Iterator<Item = u8>) -> Result<Option<PatternAttribute>> {
    let header = &match read_header_name(&mut header_iter) {
        None => return Ok(None),
        Some(x) => x,
    };
    let content = &match read_header_content(&mut header_iter) {
        None => return Ok(None),
        Some(x) => x,
    };
    match header {
        b"   " => return Ok(None),
        b"LA:" => {
            return Ok(Some(PatternAttribute::Title(
                String::from_utf8_lossy(content).into(),
            )))
        }
        b"ST:" => {
            return Ok(Some(PatternAttribute::StitchCount(
                uint_from_decimal_bytes(content)?,
            )))
        }
        b"CO:" => {
            return Ok(Some(PatternAttribute::ColorChangeCount(
                uint_from_decimal_bytes(content)?,
            )))
        }
        b"+X:" => {
            return Ok(Some(PatternAttribute::BoundsMinX(
                decimal_from_decimal_bytes(content)?,
            )))
        }

        // case cci('C','O'): /* Color change count, 3 digits padded by leading 0's */
        // case cci('+','X'): /* Design extents (+/-X,+/-Y), 5 digits padded by leading 0's */
        // case cci('-','X'):
        // case cci('+','Y'):
        // case cci('-','Y'):
        // }
        _ => {}
    }

    return Ok(None);
}

fn uint_from_decimal_bytes(items: &Vec<u8>) -> Result<u32> {
    let mut value: u32 = 0;
    for byte in items {
        if b' ' == *byte {
            continue;
        } else if b'0' <= *byte && *byte <= b'9' {
            value = (10 * value) + (*byte - b'0') as u32;
        } else {
            return Err(
                ErrorKind::InvalidFormatError("Invalid byte in header number.".to_owned()).into(),
            );
        }
    }
    return Ok(value);
}
//
// fn read_int_from_header(in_bytes: &mut Iterator<Item = u8>, num_bytes: usize) -> Result<u32> {
//     let mut value: u32 = 0;
//     for byte in in_bytes.take(num_bytes) {
//         if b'0' <= byte && byte <= b'9' {
//             value = (10 * value) + (byte - b'0') as u32;
//         } else if b' ' == byte {
//             continue;
//         } else {
//             return Err(
//                 ErrorKind::InvalidFormatError("Invalid byte in header number.".to_owned()).into(),
//             );
//         }
//     }
//     return Ok(value);
// }

fn read_header_name(in_bytes: &mut Iterator<Item = u8>) -> Option<[u8; 3]> {
    let header_bytes = Vec::from_iter(in_bytes.take(3));
    let items = header_bytes.as_slice();
    if items.len() < 3 {
        return None;
    }
    return Some([items[0], items[1], items[2]]);
}

fn read_header_content(in_bytes: &mut Iterator<Item = u8>) -> Option<Vec<u8>> {
    let mut items = Vec::new();
    for item in in_bytes {
        if item == b'\n' {
            break;
        } else {
            items.push(item)
        }
    }
    if items.len() > 0 {
        return Some(items);
    }
    return None;
}
