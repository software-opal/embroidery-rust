use std::io::Read;
use std::iter::FromIterator;

use crate::format::errors::{ReadError, ReadResult as Result};
use crate::format::traits::PatternLoader;
use crate::format::utils::ReadByteIterator;
use crate::pattern::pattern::{Pattern, PatternAttribute};
use crate::pattern::stitch::{ColorGroup, Stitch, StitchGroup};
use crate::utils::c_trim;

use crate::formats::dst::stitch_info::StitchInformation;
use crate::formats::dst::stitch_info::StitchType;

pub struct DstPatternLoader {}

#[derive(Debug, Clone, PartialEq)]
enum ParseResult<T> {
    Some(T),
    Skip,
    Exhausted,
}

impl Default for DstPatternLoader {
    fn default() -> Self {
        DstPatternLoader {}
    }
}

impl PatternLoader for DstPatternLoader {
    fn is_loadable(&self, item: &mut Read) -> Result<bool> {
        // Load the header
        // Check the last byte of the file? maybe
        let mut iter = ReadByteIterator::new(item);
        return match read_dst_header(&mut iter) {
            Err(ReadError::InvalidFormatError(_)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        };
    }

    fn read_pattern(&self, item: &mut Read) -> Result<Pattern> {
        // Read the header
        let mut iter = ReadByteIterator::new(item);
        let attributes = read_dst_header(&mut iter)?;
        let color_groups = read_stitches(&mut iter)?;
        let (title, attributes) = extract_title(attributes);
        return Ok(Pattern {
            name: title,
            attributes: attributes,
            color_groups: color_groups,
        });
    }
}

fn extract_title(attrs: Vec<PatternAttribute>) -> (String, Vec<PatternAttribute>) {
    let mut new_attrs: Vec<PatternAttribute> = Vec::new();
    let mut title = "Untitled".to_owned();
    for attr in attrs {
        if let PatternAttribute::Title(ttl) = attr {
            title = ttl;
        } else {
            new_attrs.push(attr);
        }
    }
    let title_attr = PatternAttribute::Title(title.to_owned());
    new_attrs.push(title_attr);
    return (title, new_attrs);
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
    debug!(
        "Read DST Header: {:?}:{:?}",
        String::from_utf8_lossy(header).to_string(),
        String::from_utf8_lossy(content).to_string()
    );
    match header {
        b"LA" => Ok(ParseResult::Some(PatternAttribute::Title(c_trim(
            &String::from_utf8_lossy(content),
        )))),
        b"AU" => Ok(ParseResult::Some(PatternAttribute::Author(c_trim(
            &String::from_utf8_lossy(content),
        )))),
        b"CP" => Ok(ParseResult::Some(PatternAttribute::Copyright(c_trim(
            &String::from_utf8_lossy(content),
        )))),
        // We can skip these because they're calculated from the stitches.
        b"ST" => Ok(ParseResult::Skip),
        b"CO" => Ok(ParseResult::Skip),
        b"+X" => Ok(ParseResult::Skip),
        b"+Y" => Ok(ParseResult::Skip),
        b"-X" => Ok(ParseResult::Skip),
        b"-Y" => Ok(ParseResult::Skip),
        // We can skip these because they're all related to multi-file patterns, which we don't support
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

fn read_stitches(item: &mut Iterator<Item = u8>) -> Result<Vec<ColorGroup>> {
    let mut color_groups = Vec::new();
    let mut stitch_groups = Vec::new();
    let mut stitches = Vec::new();
    let mut last_irregulars: Vec<(i32, i32, StitchType)> = Vec::new();
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    loop {
        let s = read_stitch(item);
        match s {
            ParseResult::Some(StitchInformation::Move(x, y, stitch_type)) => {
                if !last_irregulars.is_empty() && stitch_type.is_regular() {
                    debug!("Last Stitch: {:?}", stitches.last());
                    if !stitches.is_empty() {
                        let old_stitches = stitches;
                        stitches = Vec::new();
                        stitch_groups.push(StitchGroup {
                            stitches: old_stitches,
                            trim: true,
                            cut: determine_cut(&last_irregulars),
                        });
                        debug!("Last cut: {}", stitch_groups[0].cut)
                    }
                    if !stitch_groups.is_empty() {
                        if last_irregulars.iter().any(|&(_, _, ref st)| st.is_stop()) {
                            let old_stitch_groups = stitch_groups;
                            stitch_groups = Vec::new();
                            color_groups.push(ColorGroup {
                                stitch_groups: old_stitch_groups,
                                // TODO: threads
                                thread: None,
                            });
                        }
                    }
                    last_irregulars = Vec::new();
                    // First stitch after a series of jumps should be the location where the
                    // jumps ended up.
                    stitches.push(Stitch {
                        x: cx as f64 / 10.,
                        y: cy as f64 / 10.,
                    });
                }
                if !stitch_type.is_regular() && last_irregulars.is_empty() {
                    debug!("Last Regular ({:?},{:?}). Delta: {},{}", cx, cy, x, y);
                    last_irregulars.push((cx, cy, StitchType::Regular));
                }
                cx = cx + x as i32;
                cy = cy + y as i32;

                if stitch_type.is_regular() {
                    stitches.push(Stitch {
                        x: cx as f64 / 10.,
                        y: cy as f64 / 10.,
                    });
                } else {
                    debug!("Irregular {:?} {:?} {:?}", cx, cy, stitch_type);
                    last_irregulars.push((cx, cy, stitch_type));
                }
            }
            ParseResult::Some(StitchInformation::End) => {
                break;
            }
            ParseResult::Exhausted => {
                break;
            }
            ParseResult::Skip => {}
        }
    }
    if stitches.len() > 0 {
        stitch_groups.push(StitchGroup {
            stitches: stitches,
            trim: true,
            cut: determine_cut(&last_irregulars),
        });
    }
    if stitch_groups.len() > 0 {
        color_groups.push(ColorGroup {
            stitch_groups: stitch_groups,
            thread: None,
        });
    }
    return Ok(color_groups);
}

fn read_stitch(in_bytes: &mut Iterator<Item = u8>) -> ParseResult<StitchInformation> {
    let header_bytes = Vec::from_iter(in_bytes.take(3));
    let items = header_bytes.as_slice();
    if items.len() < 3 {
        ParseResult::Exhausted
    } else {
        ParseResult::Some(StitchInformation::from_bytes([
            items[0], items[1], items[2],
        ]))
    }
}

fn determine_cut(stitches: &Vec<(i32, i32, StitchType)>) -> bool {
    debug!("determine_cut {} {:?}", stitches.len(), stitches);
    for i in 0..(stitches.len()) {
        debug!("Checking for moves {}: {:?}", i, stitches.iter().nth(i));
        let mut st = stitches.iter();
        let (fx, fy, _) = if let Some(x) = st.nth(i) {
            x
        } else {
            return false;
        };
        for &(cx, cy, _) in st {
            if (fx - 1 <= cx && cx <= fx + 1) && (fy - 1 <= cy && cy <= fy + 1) {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = read_dst_header(to_u8_iter!(BASIC_HEADER_SAMPLE)).unwrap();
        let mut iter = result.iter();
        assert_eq!(
            iter.next(),
            Some(&PatternAttribute::Title("crown FS 40".to_string())),
        );
        // ST, CO, +X, -X, +Y, -Y, AX, AY, MX, MY are skipped intentionally.
        // PD is skipped intentionally.
        assert_eq!(iter.next(), None);
    }

    const BASIC_HEADER_SAMPLE: &[u8] = b"\
LA:crown FS 40     \rST:   4562\rCO:  7\r+X:  362\r\
-X:  357\r+Y:  240\r-Y:  267\rAX:+   15\rAY:-   24\r\
MX:+    0\rMY:+    0\rPD:******\r\x1a                ";
}
