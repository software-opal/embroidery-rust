use std::io::{Bytes, Error, Read};

pub struct ReadByteIterator<T: Read + Sized> {
    reader: Bytes<T>,
    pub closed: bool,
    pub error: Option<Error>,
}

impl<T: Read + Sized> ReadByteIterator<T> {
    pub fn new(reader: T) -> Self
    where
        T: Read + Sized,
    {
        Self {
            reader: reader.bytes(),
            closed: false,
            error: None,
        }
    }
    pub fn close(self: &mut Self) {
        self.closed = true
    }
}

impl<T: Read + Sized> Iterator for ReadByteIterator<T> {
    type Item = u8;
    fn next(self: &mut Self) -> Option<<Self as Iterator>::Item> {
        if self.closed {
            None
        } else {
            match self.reader.next() {
                Some(Ok(value)) => Some(value),
                Some(Err(error)) => {
                    self.error = Some(error);
                    self.close();
                    None
                },
                None => {
                    self.close();
                    None
                },
            }
        }
    }
}
