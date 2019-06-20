use byteorder::{LittleEndian, ReadBytesExt};
use embroidery_lib::format::errors::ReadResult;
use embroidery_lib::prelude::*;
use std::io::Read;

use crate::hoops::JefHoop;

use crate::colors::JEF_THREADS;

#[derive(Debug, Clone, PartialEq)]
pub struct PatternHeader {
    pub stitch_abs_offset: u32,
    pub format_flags: u32,
    pub datetime: [u8; 14],

    pub number_of_colors: u32,
    pub number_of_stitches: u32,
    pub hoop: JefHoop,

    pub bounds: (u32, u32, u32, u32),
    pub rect_from_110x110: (u32, u32, u32, u32),
    pub rect_from_50x50: (u32, u32, u32, u32),
    pub rect_from_200x140: (u32, u32, u32, u32),
    pub rect_from_custom: (u32, u32, u32, u32),

    pub threads: Vec<Thread>,
}

impl PatternHeader {
    pub fn build(file: &mut dyn Read) -> ReadResult<Self> {
        let stitch_abs_offset = file.read_u32::<LittleEndian>()?;
        let format_flags = file.read_u32::<LittleEndian>()?; /* TODO: find out what this means */
        //
        let datetime = {
            // String of: yyyymmddHHMMSS
            let mut v = [0; 14];
            file.read_exact(&mut v)?;
            v
        };
        assert_eq!(0, file.read_u16::<LittleEndian>()?);

        let number_of_colors = file.read_u32::<LittleEndian>()?;
        let number_of_stitches = file.read_u32::<LittleEndian>()?;
        let hoop = JefHoop::from_byte(file.read_u32::<LittleEndian>()?);

        let bounds = (
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
        );
        let rect_from_110x110 = (
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
        );
        let rect_from_50x50 = (
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
        );
        let rect_from_200x140 = (
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
        );
        let rect_from_custom = (
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
            file.read_u32::<LittleEndian>()?,
        );
        let mut threads = Vec::with_capacity(number_of_colors as usize);
        for _ in 0..number_of_colors {
            let idx = (file.read_u32::<LittleEndian>()? as usize) % 79;
            let (color, name, code) = JEF_THREADS[idx];
            threads.push(Thread::new_str(color, name, code))
        }
        Ok(PatternHeader {
            stitch_abs_offset,
            format_flags,
            datetime,
            number_of_colors,
            number_of_stitches,
            hoop,
            bounds,
            rect_from_110x110,
            rect_from_50x50,
            rect_from_200x140,
            rect_from_custom,
            threads,
        })
    }

    // pub fn header_len(&self) -> usize {
    //     // Magic bytes + #stitches + #colors + [+ve x] + [+ve y] + [-ve x] + [-ve y]
    //     //  + [attr offset] + [x offset] + [y offset] + [10 bytes]
    //     let header = 4 + (2 * 4) + (4 * 2) + (3 * 4) + 10;
    //     let header_color_extra = match self.pattern_type {
    //         PatternType::Hus => 0,
    //         PatternType::Vip => 4,
    //     };
    //     header + header_color_extra
    // }
    // pub fn color_len(&self) -> usize {
    //     match self.pattern_type {
    //         PatternType::Hus => (self.number_of_colors as usize) * 2,
    //         PatternType::Vip => (self.number_of_colors as usize) * 4,
    //     }
    // }
    // pub fn color_consume_len(&self) -> usize {
    //     (self.attribute_offset as usize) - self.header_len()
    // }
    //
    // pub fn attribute_len(&self) -> usize {
    //     (self.x_offset as usize) - (self.attribute_offset as usize)
    // }
    // pub fn x_offset_len(&self) -> usize {
    //     (self.y_offset as usize) - (self.x_offset as usize)
    // }
    //
    // pub fn write(&self, file: &mut dyn Write) -> Result<()> {
    //     file.write_all(&self.pattern_type.magic_bytes())?;
    //     file.write_u32::<LittleEndian>(self.number_of_stitches)?;
    //     file.write_u32::<LittleEndian>(self.number_of_colors)?;
    //     file.write_i16::<LittleEndian>(self.postitive_x_hoop_size)?;
    //     file.write_i16::<LittleEndian>(self.postitive_y_hoop_size)?;
    //     file.write_i16::<LittleEndian>(self.negative_x_hoop_size)?;
    //     file.write_i16::<LittleEndian>(self.negative_y_hoop_size)?;
    //     file.write_u32::<LittleEndian>(self.attribute_offset)?;
    //     file.write_u32::<LittleEndian>(self.x_offset)?;
    //     file.write_u32::<LittleEndian>(self.y_offset)?;
    //
    //     file.write_all(&[0x00; 10])?;
    //
    //     if self.pattern_type == PatternType::Vip {
    //         // This was derrived from a number of files; Don't understand why though.
    //         file.write_u32::<LittleEndian>(0x2E + 8 * self.number_of_colors)?;
    //     }
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let data = [
            0x84, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x32, 0x30, 0x31, 0x36, 0x31, 0x31, 0x32, 0x38, 0x31, 0x30,
            0x32, 0x34, 0x32, 0x32, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xB6, 0x04, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0xAA, 0x02, 0x00, 0x00, 0xB3, 0x02, 0x00, 0x00, 0xAA, 0x02, 0x00, 0x00, 0xB4, 0x02, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x12, 0x00, 0x00, 0x00, 0x35, 0x01,
            0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x34, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x35, 0x01, 0x00, 0x00,
            0x12, 0x00, 0x00, 0x00, 0x34, 0x01, 0x00, 0x00, 0x3B, 0x00, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00, 0x0D, 0x00,
            0x00, 0x00, 0x0D, 0x00, 0x00, 0x00,
        ];
        let header = PatternHeader::build(&mut Cursor::new(&data[..])).unwrap();
        assert_eq!(header.pattern_type, PatternType::Vip);
        assert_eq!(header.number_of_stitches, 0x00_00_03_78);
        assert_eq!(header.number_of_colors, 0x00_00_00_01);
        assert_eq!(header.postitive_x_hoop_size, 0x00_b3);
        assert_eq!(header.postitive_y_hoop_size, 0x00_b5);
        assert_eq!(header.negative_x_hoop_size, 0x4d - 0x100);
        assert_eq!(header.negative_y_hoop_size, 0x4c - 0x100);
        assert_eq!(header.attribute_offset, 0x00_00_00_4e);
        assert_eq!(header.x_offset, 0x00_00_00_6b);
        assert_eq!(header.y_offset, 0x00_00_02_8b);
        // Skip 10 bytes
        // Skip 4 bytes
        let mut out = Vec::with_capacity(data.len());
        header.write(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(&data[..], &out[..])
    }
}
