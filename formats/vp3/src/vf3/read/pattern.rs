use std::io::Read;

use byteorder::BigEndian;

use embroidery_lib::errors::add_context_fn;
use embroidery_lib::prelude::*;
use embroidery_lib::{read_exact, read_exact_magic, read_int};

use crate::common::util::{read_ascii_string_field, read_wide_string_field};

fn pair_window<'a, T>(iter: &'a [T]) -> Vec<(&'a T, Option<&'a T>)> {
    let mut next_item_iter = iter.iter().skip(1);
    let mut collect = Vec::with_capacity(iter.len());
    for item in iter {
        collect.push((item, next_item_iter.next()))
    }
    collect
}

pub fn read_font_pattern(reader: &mut dyn Read, character_offsets: &[(char, u32)]) -> Result<Vec<Pattern>, ReadError> {
    if character_offsets.is_empty() {
        return Ok(vec![]);
    }
    let bytes_read: u64 = 0;

    let mut patterns = Vec::with_capacity(character_offsets.len());

    for (i, &(&(chr, offset), next_char)) in pair_window(character_offsets).iter().enumerate() {
        let this_char_bytes = next_char
            .map(|&(_, next)| (next - offset).into())
            .unwrap_or(std::u64::MAX);
        let offset = offset.into();

        let (attributes, color_groups) = add_context_fn(
            || {
                if bytes_read > offset {
                    return Err(ReadError::invalid_format(format!(
                        "Offset was beyond the current offset. Currently at {}, expecting {}",
                        bytes_read, offset
                    )));
                }
                let mut constrained_reader = reader.take(this_char_bytes);
                read_char_pattern(&mut constrained_reader)
            },
            || format!("Error ocurred whilst processing character {:?} at index {}.", chr, i),
        )?;
        patterns.push(Pattern {
            name: format!("{}", chr),
            attributes,
            color_groups,
        });
    }

    Ok(patterns)
}

pub fn read_char_pattern(
    unconstrained_reader: &mut dyn Read,
) -> Result<(Vec<PatternAttribute>, Vec<ColorGroup>), ReadError> {
    read_exact_magic!(unconstrained_reader, [0x00, 0x11, 0x00])?;
    let length = read_int!(unconstrained_reader, u32, BigEndian)?.into();
    let reader = &mut unconstrained_reader.take(length);
    read_exact_magic!(reader, [0x33])?;
    let settings = read_wide_string_field(reader, "settings")?;
    read_exact_magic!(reader, [0x18])?;
    let software_string = read_wide_string_field(reader, "software_string")?;
    let thread_count = read_int!(reader, u16, BigEndian)?.into();

    let mut threads = Vec::with_capacity(thread_count);
    for thread_num in 0..thread_count {
        let thread = add_context_fn(
            || read_thread_wrapper(reader),
            || format!("Error ocurred whilst processing thread {}", thread_num),
        )?;

        threads.push(thread)
    }

    Ok((
        vec![
            PatternAttribute::Arbitrary("settings".to_owned(), settings),
            PatternAttribute::Arbitrary("software_string".to_owned(), software_string),
        ],
        threads,
    ))
}

fn read_thread_wrapper(reader: &mut dyn Read) -> Result<ColorGroup, ReadError> {
    let start_x = read_int!(reader, i32, BigEndian)?;
    let start_y = read_int!(reader, i32, BigEndian)?;
    let table_len = read_int!(reader, u8)?.into();
    let color = read_exact!(reader, [_; 3])?;
    let table = read_exact!(reader, vec![_; table_len])?;
    println!("Table: {:?}", table);

    let thread_number = read_ascii_string_field(reader, "thread_number")?;
    let thread_name = read_ascii_string_field(reader, "thread_name")?;
    let thread_brand = read_ascii_string_field(reader, "thread_brand")?;
    let _next_color_offset_x = read_int!(reader, i32, BigEndian)?;
    let _next_color_offset_y = read_int!(reader, i32, BigEndian)?;

    let unknown_len = read_int!(reader, u16, BigEndian)?.into();
    let unknown = read_exact!(reader, vec![_; unknown_len])?;
    println!("Thread unknown: {:?}", unknown);

    let color_bytes = read_int!(reader, u32, BigEndian)?.into();
    let stitch_groups = read_stitches(&mut reader.take(color_bytes), (start_x, start_y))?;

    Ok(ColorGroup {
        thread: Some(Thread {
            color: color.into(),
            name: thread_name,
            code: thread_number,
            manufacturer: Some(thread_brand),
            ..Thread::default()
        }),
        stitch_groups: stitch_groups,
    })
}

fn read_stitches(reader: &mut dyn Read, (mut abs_x, mut abs_y): (i32, i32)) -> Result<Vec<StitchGroup>, ReadError> {
    read_exact_magic!(reader, [0x00, 0x00, 0x00])?;

    let mut stitches = vec![];
    let stitch_groups = vec![];

    loop {
        let mut pos = [0u8; 2];
        let read = reader.read(&mut pos)?;
        if read == 0 {
            break;
        } else if read != 2 {
            return Err(ReadError::invalid_format(format!(
                "Incorrect number of bytes remaining in stitch block. Expected 0 or 2, got {}",
                read
            )));
        }
        if pos == [0x80, 0x01] {
            abs_x += vp3_u16_convert(read_int!(reader, u16, BigEndian)?);
            abs_y += vp3_u16_convert(read_int!(reader, u16, BigEndian)?);

            stitches.push(Stitch::new(abs_x.into(), abs_y.into()));
        } else if pos[0] == 0x80 {
            println!("Stitch found that has x == 0x80: {:?}", pos);
        } else {
            abs_x += vp3_u8_convert(pos[0]);
            abs_y += vp3_u8_convert(pos[0]);
            stitches.push(Stitch::new(abs_x.into(), abs_y.into()));
        }
    }

    return Ok(stitch_groups);
}

#[inline]
fn vp3_u8_convert(i: u8) -> i32 {
    if i == 0x80 {
        return 0x80;
    } else {
        return i8::from_be_bytes([i]).into();
    }
}
#[inline]
fn vp3_u16_convert(i: u16) -> i32 {
    if i == 0x8000 {
        return 0x8000;
    } else {
        return i16::from_be_bytes(i.to_be_bytes()).into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vp3_u8_convert() {
        assert_eq!(0x80, vp3_u8_convert(0x80));
        assert_eq!(-0x7f, vp3_u8_convert(0x81));
        for i in 0..=0x80 {
            assert_eq!(i32::from(i), vp3_u8_convert(i), "Input: {}", i);
        }
        for (input, output) in (0x81..=0xff).zip(-0x7f..=-0x01) {
            assert_eq!(output, vp3_u8_convert(input), "Input: {}", input);
        }
    }

    #[test]
    fn test_vp3_u16_convert() {
        assert_eq!(0x8000, vp3_u16_convert(0x8000));
        assert_eq!(-0x7fff, vp3_u16_convert(0x8001));
        for i in 0..=0x8000 {
            assert_eq!(i32::from(i), vp3_u16_convert(i), "Input: {}", i);
        }
        for (input, output) in (0x8001..=0xffff).zip(-0x7fff..=-0x0001) {
            assert_eq!(output, vp3_u16_convert(input), "Input: {}", input);
        }
    }

    #[test]
    fn test_pair_window() {
        assert_eq!(pair_window::<u8>(&[]), vec![]);
        assert_eq!(pair_window(&[1]), vec![(&1, None)]);
        assert_eq!(pair_window(&[1, 2]), vec![(&1, Some(&2)), (&2, None)]);
        assert_eq!(
            pair_window(&[1, 2, 3]),
            vec![(&1, Some(&2)), (&2, Some(&3)), (&3, None)]
        );
    }

    mod read_char_pattern {
        use super::*;

        #[test]
        fn test_send_vf3_space_character() {
            // Send.vf3  StartOffset(h): 000002CD, EndOffset(h): 00000354, Length(h): 00000088
            let data: [u8; 0x88] = [
                0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x23, 0x8C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x64, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x78, 0x78, 0x50, 0x50, 0x01, 0x00, 0x00,
                0x30, 0x00, 0x50, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x64, 0x00, 0x75, 0x00, 0x63, 0x00, 0x65, 0x00, 0x64,
                0x00, 0x20, 0x00, 0x62, 0x00, 0x79, 0x00, 0x20, 0x00, 0x56, 0x00, 0x53, 0x00, 0x4D, 0x00, 0x20, 0x00,
                0x47, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x75, 0x00, 0x70, 0x00, 0x20, 0x00, 0x41, 0x00, 0x42, 0x00, 0x00,
            ];
            let reader = &mut &data[..];

            let (attrs, pattern) = read_char_pattern(reader).unwrap();
            assert_eq!(reader, &[]);
            assert_eq!(attrs, vec![]);
            assert_eq!(pattern, vec![]);
        }

        #[test]
        fn test_send_vf3_exclamation_character() {
            // Send.vf3  StartOffset(h): 00000355, EndOffset(h): 00000495, Length(h): 00000141
            let data: [u8; 0x141] = [
                0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x3A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x13, 0xEC, 0xFF, 0xFF, 0xF3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xEF, 0xFC, 0x00, 0x00, 0x10, 0x04, 0xFF, 0xFF, 0xCE, 0x32, 0x00, 0x00, 0x31, 0xCE, 0x00,
                0x00, 0x20, 0x08, 0x00, 0x00, 0x63, 0x9C, 0x00, 0x00, 0x64, 0x64, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x78, 0x78, 0x50, 0x50, 0x01, 0x00, 0x00,
                0x30, 0x00, 0x50, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x64, 0x00, 0x75, 0x00, 0x63, 0x00, 0x65, 0x00, 0x64,
                0x00, 0x20, 0x00, 0x62, 0x00, 0x79, 0x00, 0x20, 0x00, 0x56, 0x00, 0x53, 0x00, 0x4D, 0x00, 0x20, 0x00,
                0x47, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x75, 0x00, 0x70, 0x00, 0x20, 0x00, 0x41, 0x00, 0x42, 0x00, 0x01,
                0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0xB2, 0xFF, 0xFF, 0xEF, 0xFC, 0xFF, 0xFF, 0xD0, 0x26, 0x01, 0x00,
                0x0B, 0xC1, 0xD7, 0x00, 0x00, 0x00, 0x05, 0x28, 0x00, 0x04, 0x32, 0x35, 0x31, 0x38, 0x00, 0x11, 0x49,
                0x6E, 0x64, 0x69, 0x61, 0x6E, 0x20, 0x4F, 0x63, 0x65, 0x61, 0x6E, 0x20, 0x42, 0x6C, 0x75, 0x65, 0x00,
                0x16, 0x52, 0x6F, 0x62, 0x69, 0x73, 0x6F, 0x6E, 0x2D, 0x41, 0x6E, 0x74, 0x6F, 0x6E, 0x20, 0x52, 0x61,
                0x79, 0x6F, 0x6E, 0x20, 0x34, 0x30, 0x00, 0x00, 0x1C, 0x20, 0x00, 0x00, 0x0C, 0x80, 0x00, 0x01, 0x00,
                0x00, 0x00, 0x00, 0x5F, 0x0A, 0xF6, 0x00, 0x39, 0xC0, 0x03, 0x00, 0xFE, 0x00, 0x04, 0x00, 0xFC, 0x00,
                0xFD, 0xE7, 0x00, 0xFB, 0xFD, 0xE7, 0x00, 0xFC, 0xFD, 0xE7, 0xFF, 0xFC, 0xFD, 0xE7, 0x00, 0xFC, 0xFD,
                0xE7, 0x00, 0xFB, 0x01, 0xE7, 0x15, 0xF2, 0x11, 0x12, 0x01, 0x15, 0xFD, 0x19, 0xFF, 0x05, 0xFD, 0x19,
                0x00, 0x04, 0xFD, 0x19, 0x00, 0x04, 0xFD, 0x19, 0x00, 0x04, 0xFD, 0x19, 0xFD, 0x05, 0xFA, 0x00, 0x03,
                0x00, 0xFE, 0x00, 0x04, 0x00, 0x0D, 0x23, 0xFE, 0xFE, 0x01, 0x01, 0xFE, 0xFE, 0x08, 0x18, 0xEB, 0x0D,
                0xEF, 0xEE, 0x0D, 0xEB, 0x10, 0x01, 0x04, 0x04, 0xFE, 0xFE, 0x01, 0x01, 0xFE, 0xFE, 0x00,
            ];
            let reader = &mut &data[..];

            let (attrs, pattern) = read_char_pattern(reader).unwrap();
            assert_eq!(reader, &[]);
            assert_eq!(attrs, vec![]);
            assert_eq!(pattern, vec![]);
        }
    }
}
