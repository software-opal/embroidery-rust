use pattern::stitch::StitchGroup;
use std::io::Read;
use std::iter::FromIterator;

use formats::errors::{Error, ErrorKind, Result};
use formats::traits::PatternLoader;
use formats::utils::{is_byte_set, ReadByteIterator};
use pattern::pattern::{Pattern, PatternAttribute};

pub struct DstPatternLoader {}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseResult<T> {
    Some(T),
    Skip,
    Exhausted,
}

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
            ParseResult::Some(header) => attrs.push(header),
            ParseResult::Skip => (),
            _ => break,
        }
    }
    // Drain the rest of the iterator.
    header_iter.last();
    return Ok(attrs);
}

fn read_header_item(
    mut header_iter: &mut Iterator<Item = u8>,
) -> Result<ParseResult<PatternAttribute>> {
    let header = &match read_header_name(&mut header_iter) {
        ParseResult::Some(header) => header,
        ParseResult::Skip => return Ok(ParseResult::Skip),
        ParseResult::Exhausted => return Ok(ParseResult::Exhausted),
    };
    let content = &match read_header_content(&mut header_iter) {
        ParseResult::Some(content) => content,
        ParseResult::Skip => return Ok(ParseResult::Skip),
        ParseResult::Exhausted => return Ok(ParseResult::Exhausted),
    };
    match header {
        b"LA" => Ok(ParseResult::Some(PatternAttribute::Title(
            String::from_utf8_lossy(content).trim().to_string(),
        ))),
        // We can skip these because they're calculated from the stitches.
        b"ST" => Ok(ParseResult::Skip),
        b"CO" => Ok(ParseResult::Skip),
        b"+X" => Ok(ParseResult::Skip),
        b"+Y" => Ok(ParseResult::Skip),
        b"-X" => Ok(ParseResult::Skip),
        b"-Y" => Ok(ParseResult::Skip),
        b"AX" => Ok(ParseResult::Skip),
        b"AY" => Ok(ParseResult::Skip),
        b"MX" => Ok(ParseResult::Skip),
        b"MY" => Ok(ParseResult::Skip),
        b"PD" => Ok(ParseResult::Skip),
        _ => Ok(ParseResult::Some(PatternAttribute::Arbitary(
            String::from_utf8_lossy(header).to_string(),
            String::from_utf8_lossy(content).to_string(),
        ))),
    }
}

fn read_header_name(in_bytes: &mut Iterator<Item = u8>) -> ParseResult<[u8; 2]> {
    let header_bytes = Vec::from_iter(in_bytes.take(3));
    let items = header_bytes.as_slice();
    if items.len() < 3 {
        return ParseResult::Exhausted;
    } else if items[2] != b':' && items[2] != b'*' {
        return ParseResult::Skip;
    }
    return ParseResult::Some([items[0], items[1]]);
}

fn read_header_content(in_bytes: &mut Iterator<Item = u8>) -> ParseResult<Vec<u8>> {
    let mut items = Vec::new();
    let mut exhausted = true;
    for item in in_bytes {
        if item == b'\r' {
            exhausted = false;
            break;
        } else {
            items.push(item)
        }
    }
    if !exhausted || items.len() > 0 {
        ParseResult::Some(items)
    } else {
        ParseResult::Exhausted
    }
}

enum StitchInformation {
    Move {
        dx: i8,
        dy: i8,
        stop: bool,
        jump: bool,
    },
    End,
    Invalid,
}

impl StitchInformation {
    fn from_bytes(b1: u8, b2: u8, b3: u8) -> StitchInformation {
        if b1 == 0x00 && b2 == 0x00 && b3 == 0xF3 {
            return StitchInformation::End;
        }
        let bytes: u64 = ((b1 as u64) << 16) | ((b2 as u64) << 8) | ((b3 as u64) << 0);
        if !is_byte_set(1, bytes) || !is_byte_set(0, bytes) {
            return StitchInformation::Invalid;
        }
        let stop_flag = is_byte_set(6, bytes);
        let jump_flag = is_byte_set(7, bytes);
        let dx: i8 = 0;
        let dy: i8 = 0;
        dx += if is_byte_set(16, bytes) { 1 } else { 0 };
        dx += if is_byte_set(17, bytes) { -1 } else { 0 };
        dx += if is_byte_set(8, bytes) { 3 } else { 0 };
        dx += if is_byte_set(9, bytes) { -3 } else { 0 };
        dx += if is_byte_set(18, bytes) { 9 } else { 0 };
        dx += if is_byte_set(19, bytes) { -9 } else { 0 };
        dx += if is_byte_set(10, bytes) { 27 } else { 0 };
        dx += if is_byte_set(11, bytes) { -27 } else { 0 };
        dx += if is_byte_set(2, bytes) { 81 } else { 0 };
        dx += if is_byte_set(3, bytes) { -81 } else { 0 };

        dy += if is_byte_set(23, bytes) { 1 } else { 0 };
        dy += if is_byte_set(22, bytes) { -1 } else { 0 };
        dy += if is_byte_set(15, bytes) { 3 } else { 0 };
        dy += if is_byte_set(14, bytes) { -3 } else { 0 };
        dy += if is_byte_set(21, bytes) { 9 } else { 0 };
        dy += if is_byte_set(20, bytes) { -9 } else { 0 };
        dy += if is_byte_set(13, bytes) { 27 } else { 0 };
        dy += if is_byte_set(12, bytes) { -27 } else { 0 };
        dy += if is_byte_set(5, bytes) { 81 } else { 0 };
        dy += if is_byte_set(4, bytes) { -81 } else { 0 };

        return StitchInformation::Move {
            dx,
            dy,
            stop: stop_flag,
            jump: jump_flag,
        };
    }
}

fn read_stitches(item: &mut Iterator<Item = u8>) {
    let stitches: &mut Vec<StitchGroup> = vec![];
    let mut current_group: Option<StitchGroup> = None;
    loop {
        let stitch_bytes = Vec::from_iter(item.take(3));
        if stitch_bytes.len() != 3 {
            // Only reaches here when the iterator hasn't ended correctly(according to the spec).
            return;
        }
        let b1: u8 = stitch_bytes[0];
        let b2: u8 = stitch_bytes[1];
        let b3: u8 = stitch_bytes[2];

        let stitch = StitchInformation::from_bytes(b1, b2, b3);
        match stitch {
            StitchInformation::Invalid => return,
            StitchInformation::End => break,
            StitchInformation::Move {
                dx,
                dy,
                stop: false,
                jump: false,
            } => {
                // Regular stitch

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use formats::dst::*;

    macro_rules! to_u8_iter {
        ($t:expr) => {
            &mut $t.iter().map(|&x| x)
        };
    }

    #[test]
    fn test_read_header_name() {
        // Less than 3 bytes means the iterator has been exhausted.
        assert_eq!(read_header_name(to_u8_iter!(b"ab")), ParseResult::Exhausted);

        let iter = to_u8_iter!(b"ab:d");
        assert_eq!(read_header_name(iter), ParseResult::Some([b'a', b'b']));
        // ':' is read by the read_header_name function
        assert_eq!(iter.next(), Some(b'd'));

        let iter = to_u8_iter!(b"ab*d");
        assert_eq!(read_header_name(iter), ParseResult::Some([b'a', b'b']));
        // '*' is read by the read_header_name function
        assert_eq!(iter.next(), Some(b'd'));

        let iter = to_u8_iter!(b"abc\rd");
        // "abc" is an invalid header as it doesn't end with a ':'
        assert_eq!(read_header_name(iter), ParseResult::Skip);
    }

    #[test]
    fn test_read_header_content() {
        // An empty iterator gets exhausted
        assert_eq!(
            read_header_content(to_u8_iter!(b"")),
            ParseResult::Exhausted
        );
        // A `CR` means the end of a value, so it should be an empty value
        assert_eq!(
            read_header_content(to_u8_iter!(b"\r")),
            ParseResult::Some(vec![])
        );

        assert_eq!(
            read_header_content(to_u8_iter!(b"ab")),
            ParseResult::Some(vec![b'a', b'b'])
        );

        let iter = to_u8_iter!(b"abc\rd");
        assert_eq!(
            read_header_content(iter),
            ParseResult::Some(vec![b'a', b'b', b'c'])
        );
        assert_eq!(iter.next(), Some(b'd'));
    }

    // TODO: Find more complex examples, possibly including: thread colours, copyright, author strings.
    #[test]
    fn test_read_dst_header() {
        // Taken from `tests/dst/crown.dst`
        let BASIC_HEADER_SAMPLE = b"\
LA:crown FS 40     \rST:   4562\rCO:  7\r+X:  362\r\
-X:  357\r+Y:  240\r-Y:  267\rAX:+   15\rAY:-   24\r\
MX:+    0\rMY:+    0\rPD:******\r\x1a                ";

        let result = read_dst_header(to_u8_iter!(HEADER_SAMPLE)).unwrap();
        let mut iter = result.iter();
        assert_eq!(
            iter.next(),
            Some(&PatternAttribute::Title("crown FS 40".to_string())),
        );
        // ST, CO, +X, -X, +Y, -Y, AX, AY, MX, MY are skipped intentionally.
        // PD is skipped intentionally.
        assert_eq!(iter.next(), None);
    }

}
