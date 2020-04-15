use byteorder::LittleEndian;
use embroidery_lib::errors::ReadResult;
use embroidery_lib::prelude::*;
use embroidery_lib::read_int;
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
    #[allow(clippy::cognitive_complexity)]
    pub fn build(file: &mut dyn Read) -> ReadResult<Self> {
        let stitch_abs_offset = read_int!(file, u32, LittleEndian)?;
        let format_flags = read_int!(file, u32, LittleEndian)?; /* TODO: find out what this means */
        //
        let datetime = {
            // String of: yyyymmddHHMMSS
            let mut v = [0; 14];
            file.read_exact(&mut v)?;
            v
        };
        assert_eq!(0, read_int!(file, u16, LittleEndian)?);

        let number_of_colors = read_int!(file, u32, LittleEndian)?;
        let number_of_stitches = read_int!(file, u32, LittleEndian)?;
        let hoop = JefHoop::from_byte(read_int!(file, u32, LittleEndian)?);

        let bounds = (
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
        );
        let rect_from_110x110 = (
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
        );
        let rect_from_50x50 = (
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
        );
        let rect_from_200x140 = (
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
        );
        let rect_from_custom = (
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
            read_int!(file, u32, LittleEndian)?,
        );
        let mut threads = Vec::with_capacity(number_of_colors as usize);
        for _ in 0..number_of_colors {
            let idx = (read_int!(file, u32, LittleEndian)? as usize) % 79;
            let (color, name, code) = JEF_THREADS[idx];
            threads.push(Thread::new_str(color, &name, &code))
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
