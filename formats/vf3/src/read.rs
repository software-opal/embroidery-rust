use std::io::Read;

use embroidery_lib::format::CollectionReader;
use embroidery_lib::prelude::*;
use embroidery_lib::utils::ReadByteIterator;

pub struct Vf3CollectionReader {}

impl Default for Vf3CollectionReader {
    fn default() -> Self {
        Vf3CollectionReader {}
    }
}

impl CollectionReader for Vf3CollectionReader {
    fn is_loadable(&self, item: &mut dyn Read) -> Result<bool, ReadError> {
        // Load the header
        // Check the last byte of the file? maybe
        let mut iter = ReadByteIterator::new(item);
        Ok(false)
    }

    fn read_pattern(&self, file: &mut dyn Read) -> Result<PatternCollection, ReadError> {
        // Read the header
        let mut iter = ReadByteIterator::new(file);
        // TODO: This
        return Err(ReadError::InvalidFormat("oops".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_parsable() {
        // Less than 3 bytes means the iterator has been exhausted.
        let mut b = b"ab";
        assert_eq!(Vf3CollectionReader::default().is_loadable(&mut &b[..]).unwrap(), false);
    }
}
