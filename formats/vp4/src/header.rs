use embroidery_lib::prelude::*;
use embroidery_lib::format::traits::PatternReader;
use std::io::Read;

const MAGIC_BYTES: [u8; 6] = [b'%', b'V', b'P', b'4', b'%', 0x01];

#[derive(Default)]
pub struct Vp4PatternReader {}

impl PatternReader for Vp4PatternReader {
    fn is_loadable(&self, item: &mut dyn Read) -> Result<bool, ReadError> {
        let mut buf = [0u8; 6 /* MAGIC_BYTES.len() */];
        item.read_exact(&mut buf)?;
        Ok(buf == MAGIC_BYTES)
    }

    fn read_pattern(&self, item: &mut dyn Read) -> Result<Pattern, ReadError> {
        Err(ReadError::InvalidFormat("AAA".to_owned()))
    }
}
