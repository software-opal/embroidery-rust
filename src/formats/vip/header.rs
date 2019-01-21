use crate::format::errors::{ReadError, ReadResult};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};
use super::consts::VIP_MAGIC_BYTES
;
#[derive(Debug)]
pub struct VipHeader {
    // Magic bytes: [0x5d ,0xfc 0x90 ,0x01]
    pub number_of_stitches: u32,
    pub number_of_colors: u32,
    pub postitive_x_hoop_size: i16,
    pub postitive_y_hoop_size: i16,
    pub negative_x_hoop_size: i16,
    pub negative_y_hoop_size: i16,
    pub attribute_offset: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    // [0x00; 10]
    pub color_length: u32,
}

impl VipHeader {
    pub fn build(file: &mut dyn Read) -> ReadResult<Self> {
        {
            let mut magic_code = [0; 4];
            file.read_exact(&mut magic_code)?;
            if magic_code != VIP_MAGIC_BYTES {
                return Err(ReadError::InvalidFormatError(format!(
                    "Incorrect magic bytes: {:?}",
                    magic_code
                )));
            }
        }

        let number_of_stitches = file.read_u32::<LittleEndian>()?;
        let number_of_colors = file.read_u32::<LittleEndian>()?;

        let postitive_x_hoop_size = file.read_i16::<LittleEndian>()?;
        let postitive_y_hoop_size = file.read_i16::<LittleEndian>()?;
        let negative_x_hoop_size = file.read_i16::<LittleEndian>()?;
        let negative_y_hoop_size = file.read_i16::<LittleEndian>()?;

        let attribute_offset = file.read_u32::<LittleEndian>()?;
        let x_offset = file.read_u32::<LittleEndian>()?;
        let y_offset = file.read_u32::<LittleEndian>()?;

        {
            let mut unknown_field = [0; 10];
            file.read_exact(&mut unknown_field)?;
            if unknown_field == [0; 10] {
                return Err(ReadError::InvalidFormatError(format!(
                    "Unknown field is not blank: {:?}",
                    unknown_field
                )));
            }
        }

        let color_length = file.read_u32::<LittleEndian>()?;

        // assert_eq!(color_length, 0x38 + (number_of_colors << 3));

        Ok(VipHeader {
            number_of_stitches,
            number_of_colors,
            postitive_x_hoop_size,
            postitive_y_hoop_size,
            negative_x_hoop_size,
            negative_y_hoop_size,
            attribute_offset,
            x_offset,
            y_offset,
            color_length,
        })
    }

    pub fn write(self, file: &mut dyn Write) -> Result<()> {
        file.write_all(&VIP_MAGIC_BYTES)?;
        file.write_u32::<LittleEndian>(self.number_of_stitches)?;
        file.write_u32::<LittleEndian>(self.number_of_colors)?;
        file.write_i16::<LittleEndian>(self.postitive_x_hoop_size)?;
        file.write_i16::<LittleEndian>(self.postitive_y_hoop_size)?;
        file.write_i16::<LittleEndian>(self.negative_x_hoop_size)?;
        file.write_i16::<LittleEndian>(self.negative_y_hoop_size)?;
        file.write_u32::<LittleEndian>(self.attribute_offset)?;
        file.write_u32::<LittleEndian>(self.x_offset)?;
        file.write_u32::<LittleEndian>(self.y_offset)?;

        file.write_all(&[0x00; 10])?;

        file.write_u32::<LittleEndian>(self.color_length)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let data = [
            0x5d, 0xfc, 0x90, 0x01, 0x78, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb3, 0x00,
            0xb5, 0x00, 0x4d, 0xff, 0x4c, 0xff, 0x4e, 0x00, 0x00, 0x00, 0x6b, 0x00, 0x00, 0x00,
            0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x36, 0x00, 0x00, 0x00,
        ];
        let header = VipHeader::build(&mut Cursor::new(&data[..])).unwrap();
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
        assert_eq!(header.color_length, 0x00_00_00_36);
        let mut out = Vec::with_capacity(data.len());
        header.write(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(&data[..], &out[..])
    }
}
