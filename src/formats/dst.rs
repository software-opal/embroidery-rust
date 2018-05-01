use std::io::Read;
use std::iter::FromIterator;

use formats::errors::{Error, ErrorKind, Result};
use formats::traits::PatternLoader;
use formats::utils::ReadByteIterator;
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
        // We can skip these because they're calculated from the stitches .
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

#[cfg(test)]
mod tests {
    use formats::dst::*;

    // Taken from `tests/dst/Crown.DST`
    static BASIC_HEADER_SAMPLE: &[u8] = b"\
LA:crown FS 40     \rST:   4562\rCO:  7\r+X:  362\r\
-X:  357\r+Y:  240\r-Y:  267\rAX:+   15\rAY:-   24\r\
MX:+    0\rMY:+    0\rPD:******\r\x1a                ";

    // TODO: Find more complex examples, possibly including: thread colours, copyright, author strings.

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

    #[test]
    fn test_read_dst_header() {
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
