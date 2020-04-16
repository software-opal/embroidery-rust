use byteorder::BigEndian;

use embroidery_lib::prelude::*;
use embroidery_lib::{read_exact_magic, read_int};

use std::io::Read;

use crate::common::util::{read_ascii_string_field, read_wide_string_field};

#[derive(Debug, PartialEq)]
pub struct Vf3Header {
    pub font_name: String,
    pub character_encoding: String,
    pub character_offsets: Vec<(char, u32)>,
}

// #[derive(Debug, PartialEq)]
// pub struct Vf3Hoop {
//     pub right: i32,
//     pub bottom: i32,
//     pub left: i32,
//     pub top: i32,
//     pub unknown_a: u32,
//     pub unknown_b: u16,
//     pub bytes_remaining: usize,
//     pub x_offset: i32,
//     pub y_offset: i32,

//     /* Centered hoop dimensions */
//     pub right2: i32,
//     pub left2: i32,
//     pub bottom2: i32,
//     pub top2: i32,

//     pub width: i32,
//     pub height: i32,
// }

pub fn read_font_header(mut reader: &mut dyn Read) -> Result<Vf3Header, ReadError> {
    let font_name = read_wide_string_field(&mut reader, "font_name")?;
    let character_encoding = read_ascii_string_field(&mut reader, "character_encoding")?;

    // TODO: These aren't magic bytes but probably have some meaning.
    read_exact_magic!(
        reader,
        [
            0x00, 0x19, 0x00, 0x33, 0x42, 0x3E, 0x18, 0x02, 0xB3, 0x93, 0x48, 0x8F, 0x52, 0x89, 0x51, 0xE3, 0x78, 0xBA,
            0x9A, 0x00, 0x22, 0x00, 0x23
        ]
    )?;

    let character_count: usize = read_int!(reader, u16, BigEndian)?.into();
    let mut character_offsets = vec![('\0', 0); character_count];
    for (character, offset) in character_offsets.iter_mut() {
        let char_code = read_int!(reader, u16, BigEndian)?;
        *offset = read_int!(reader, u32, BigEndian)?;
    }

    // This is noted as [0x78, 0x78, 0x55, 0x55, 0x01, 0x00] in Embroidermodder; but testing
    // reveals it to be [0x78, 0x78, 0x50, 0x50, 0x01, 0x00]
    read_exact_magic!(reader, [0x78, 0x78, 0x50, 0x50, 0x01, 0x00])?;

    let another_software_vendor_string = read_wide_string_field(&mut reader, "another_software_vendor_string")?;

    let number_of_threads: usize = read_int!(reader, u16, BigEndian)?.into();

    Ok(Vf3Header {
        font_name,
        character_encoding,
        character_offsets,
    })
}
