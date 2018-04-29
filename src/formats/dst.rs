use std::io::Read;
use std::iter::FromIterator;

use bigdecimal::BigDecimal;

use formats::errors::{Error, ErrorKind, Result};
use formats::traits::PatternLoader;
use formats::utils::ReadByteIterator;
use pattern::pattern::{Pattern, PatternAttribute};

pub struct DstPatternLoader {}

impl PatternLoader for DstPatternLoader {
    fn is_loadable(&self, item: &mut Read) -> Result<bool> {
        // Load the header
        // Check the last byte of the file? maybe
        let mut iter = ReadByteIterator::new(item);
        return match read_dst_header(&mut iter) {
            Err(Error(ErrorKind::InvalidFormatError(_), _)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        };
    }

    fn read_pattern(&self, item: &mut Read) -> Result<Pattern> {
        // Read the header
        let mut iter = ReadByteIterator::new(item);
        read_dst_header(&mut iter)?;
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
        b"LA:" => Ok(Some(PatternAttribute::Title(
            String::from_utf8_lossy(content).into(),
        ))),
        b"ST:" => Ok(Some(PatternAttribute::StitchCount(
            uint_from_decimal_bytes(content)?,
        ))),
        b"CO:" => Ok(Some(PatternAttribute::ColorChangeCount(
            uint_from_decimal_bytes(content)?,
        ))),
        b"+X:" => Ok(Some(PatternAttribute::BoundsMinX(
            decimal_from_decimal_bytes(content)?,
        ))),
        b"+Y:" => Ok(Some(PatternAttribute::BoundsMinY(
            decimal_from_decimal_bytes(content)?,
        ))),
        b"-X:" => Ok(Some(PatternAttribute::BoundsMaxX(
            decimal_from_decimal_bytes(content)?,
        ))),
        b"-Y:" => Ok(Some(PatternAttribute::BoundsMaxY(
            decimal_from_decimal_bytes(content)?,
        ))),
        _ => Ok(None),
    }
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
    Ok(value)
}

fn decimal_from_decimal_bytes(items: &Vec<u8>) -> Result<BigDecimal> {
    let mut items_copy = items.clone();
    let decimal_pos = match items.iter().position(|&s| s == b'.') {
        None => items.len(),
        Some(idx) => {
            items_copy.remove(idx);
            items.len() - idx
        }
    };
    match uint_from_decimal_bytes(&items_copy) {
        Ok(value) => Ok(BigDecimal::new(value.into(), decimal_pos as i64)),
        Err(error) => Err(error),
    }
}

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
        if item == b'\r' {
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

#[cfg(test)]
mod tests {
    use formats::dst::*;

    static HEADER_SAMPLE: &[u8] = (b"LA:crown FS 40     \rST:   4562\rCO:  7\r+X:  362\r"
        + b"-X:  357\r+Y:  240\r-Y:  267\rAX:+   15\rAY:-   24\r"
        + b"MX:+    0\rMY:+    0\rPD:******\r\32                "
        + b"                                               "
        + b"                                               "
        + b"                                               "
        + b"                                               "
        + b"                                               "
        + b"                                               "
        + b"                                               "
        + b"                                          ");

    macro_rules! to_u8_iter {
        ($t:expr) => {
            &mut $t.iter().map(|&x| x)
        };
    }

    #[test]
    fn test_read_header_name() {
        assert_eq!(read_header_name(to_u8_iter!(b"ab")), None);

        let iter = to_u8_iter!(b"abcd");
        assert_eq!(read_header_name(iter), Some([b'a', b'b', b'c']));
        assert_eq!(iter.next(), Some(b'd'));
    }

    #[test]
    fn test_read_header_content() {
        assert_eq!(read_header_content(to_u8_iter!(b"")), None);
        assert_eq!(read_header_content(to_u8_iter!(b"\r")), None);

        assert_eq!(
            read_header_content(to_u8_iter!(b"ab")),
            Some(vec![b'a', b'b'])
        );

        let iter = to_u8_iter!(b"abc\rd");
        assert_eq!(read_header_content(iter), Some(vec![b'a', b'b', b'c']));
        assert_eq!(iter.next(), Some(b'd'));
    }

}
