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

#[macro_export]
macro_rules! read_exact_magic {
    ($reader: expr, $expected: expr) => {{
        let expected = $expected;
        // Make real the same type as expected; hopefully an array.
        let mut real = vec![0_u8; expected.len()];

        match $crate::maybe_read_with_context!(
            $crate::read_exact!($reader, &mut real),
            "Trying to read magic bytes {:?}",
            expected
        ) {
            Err(err) => Err($crate::errors::ReadError::from(err)),
            Ok(()) => {
                if real[..] != expected[..] {
                    Err($crate::errors::ReadError::invalid_format(format!(
                        "Magic bytes do not match. Expected {:?}; got {:?} at {}:{}",
                        expected,
                        real,
                        file!(),
                        line!()
                    )))
                } else {
                    // Return the read value in case the caller wants it.
                    Ok(real)
                }
            },
        }
    }};
}

#[macro_export]
macro_rules! maybe_read_with_context {
    ($maybe_error: expr, $($arg: tt)+) => {{
        use $crate::errors::ErrorWithContext;

        match $maybe_error {
            Ok(x) => Ok(x),
            Err(err) => Err(
                $crate::errors::ReadError::from(err).with_additional_context(
                    format!("{} at {}:{}", format!($($arg)+), file!(), line!()),
                )
            ),
        }
    }};
    ($maybe_error: expr) => {{
        use $crate::errors::ErrorWithContext;

        match $maybe_error {
            Ok(x) => Ok(x),
            Err(err) => Err($crate::errors::ReadError::from(err).with_additional_context(
                format!("... at {}:{}", file!(), line!()),
            )),
        }
    }};
}

#[macro_export]
macro_rules! __emb_lib_handle_eof {
    ($result: expr, $err_str: expr, $($arg : tt)*) => {
        match $result {
           Ok(x) => Ok(x),
           Err(e) => {
               if e.kind() == std::io::ErrorKind::UnexpectedEof {
                   Err($crate::errors::ReadError::unexpected_eof(
                       format!("{} at {}:{}", format!($err_str, $($arg)*), file!(), line!()),
                       e,
                   ))
               } else {
                   Err(e.into())
               }
           },
       }
    }
}

#[macro_export]
macro_rules! read_exact {
    ($reader: expr, vec![_; $length: expr]) => {{
        let mut target = vec![0_u8; $length];
        match $crate::read_exact!($reader, &mut target[..]) {
            Ok(()) => Ok(target),
            Err(e) => Err(e),
        }
    }};
    ($reader: expr, [_; $length: expr]) => {{
        let mut target = [0_u8; $length];
        match $crate::read_exact!($reader, &mut target[..]) {
            Ok(()) => Ok(target),
            Err(e) => Err(e),
        }
    }};
    ($reader: expr, $target: expr) => {{
        let real = $target;
        $crate::__emb_lib_handle_eof!(
            $reader.read_exact(real),
            "Failed to read exactly `{}` bytes from file(`{}`) into `{}`",
            real.len(),
            stringify!($reader),
            stringify!($target)
        )
    }};
}

#[macro_export]
macro_rules! read_int {
    ($reader: expr, u8) => {
        $crate::__emb_lib_read_int_impl!($reader, u8, read_u8)
    };
    ($reader: expr, i8) => {
        $crate::__emb_lib_read_int_impl!($reader, i8, read_i8)
    };

    ($reader: expr, u16, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u16, read_u16::<$endianness>)
    };
    ($reader: expr, u32, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u32, read_u32::<$endianness>)
    };
    ($reader: expr, u64, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u64, read_u64::<$endianness>)
    };
    ($reader: expr, u128, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u128, read_u128::<$endianness>)
    };
    ($reader: expr, i16, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i16, read_i16::<$endianness>)
    };
    ($reader: expr, i32, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i32, read_i32::<$endianness>)
    };
    ($reader: expr, i64, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i64, read_i64::<$endianness>)
    };
    ($reader: expr, i128, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i128, read_i128::<$endianness>)
    };
}

#[macro_export]
macro_rules! __emb_lib_read_int_impl {
    ($reader: expr, $type: ident, $($fn: tt)+) => {{
        use byteorder::ReadBytesExt;

        $crate::__emb_lib_handle_eof!(
            $reader. $($fn)+ (),
            "Failed to read `{}` from file(var name `{}`)",
            stringify!($type),
            stringify!($reader)
       )
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_exact_magic() {
        let mut reader = &([0_u8, 1, 2, 3])[..];
        assert_eq!(read_exact_magic!(&mut reader, [0, 1]).unwrap(), [0, 1]);

        let err_str = match read_exact_magic!(&mut reader, [99, 98]).unwrap_err() {
            crate::errors::ReadError::InvalidFormat(err_str, _) => err_str,
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            err_str.find("[99, 98]").is_some(),
            "Cannot find expected value in error string {:?}",
            err_str
        );
        assert!(
            err_str.find("[2, 3]").is_some(),
            "Cannot find actual value in error string {:?}",
            err_str
        );
        let (err_str, ctx) = match read_exact_magic!(&mut reader, [50, 51]).unwrap_err() {
            crate::errors::ReadError::UnexpectedEof(err_str, _, ctx) => (err_str, ctx),
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            ctx[0].find("[50, 51]").is_some(),
            "Cannot find expected value in error string {:?}",
            err_str
        );
    }

    #[test]
    fn test_read_exact() {
        let mut reader = &([0_u8, 1, 2])[..];
        let mut target = [0_u8; 2];
        read_exact!(&mut reader, &mut target).unwrap();

        let err_str = match read_exact!(&mut reader, &mut target).unwrap_err() {
            crate::errors::ReadError::UnexpectedEof(err_str, _, _) => err_str,
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            err_str.find("`2` bytes").is_some(),
            "Cannot find expected read amount in error string {:?}",
            err_str
        );
        assert!(
            err_str.find("&mut target").is_some(),
            "Cannot find target variable in error string {:?}",
            err_str
        );
    }
    #[test]
    fn test_read_exact_slice() {
        let mut reader = &([0_u8, 1, 2])[..];
        assert_eq!(read_exact!(&mut reader, [_; 2]).unwrap(), [0, 1]);
    }
    #[test]
    fn test_read_exact_vec() {
        let mut reader = &([0_u8, 1, 2])[..];
        let len = 2;
        assert_eq!(read_exact!(&mut reader, vec![_; len]).unwrap(), vec![0, 1]);
    }
    #[test]
    fn test_read_int() {
        use byteorder::BigEndian;

        // Should need (31 * 2) for the fixed sized ints and at most (16*2) for the *size ints
        // Which is 94; but use 200 just to be safe
        let mut reader = &([0_u8; 200])[..];

        assert_eq!(read_int!(&mut reader, u8).unwrap(), 0_u8);
        assert_eq!(read_int!(&mut reader, i8).unwrap(), 0_i8);

        assert_eq!(read_int!(&mut reader, u16, BigEndian).unwrap(), 0_u16);
        assert_eq!(read_int!(&mut reader, u32, BigEndian).unwrap(), 0_u32);
        assert_eq!(read_int!(&mut reader, u64, BigEndian).unwrap(), 0_u64);
        assert_eq!(read_int!(&mut reader, u128, BigEndian).unwrap(), 0_u128);
        assert_eq!(read_int!(&mut reader, i16, BigEndian).unwrap(), 0_i16);
        assert_eq!(read_int!(&mut reader, i32, BigEndian).unwrap(), 0_i32);
        assert_eq!(read_int!(&mut reader, i64, BigEndian).unwrap(), 0_i64);
        assert_eq!(read_int!(&mut reader, i128, BigEndian).unwrap(), 0_i128);
    }
}
