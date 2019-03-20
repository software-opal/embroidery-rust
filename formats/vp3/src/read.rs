use embroidery_lib::format::PatternReader;
use embroidery_lib::maybe_read_with_context;
use embroidery_lib::prelude::*;

use std::io::Read;

mod header;
mod thread;
mod util;

#[derive(Default)]
pub struct Vp3PatternReader {}

impl PatternReader for Vp3PatternReader {
    fn is_loadable(&self, reader: &mut dyn Read) -> Result<bool, ReadError> {
        header::read_header(reader)?;
        Ok(false)
    }
    fn read_pattern(&self, ub_reader: &mut dyn Read) -> Result<Pattern, ReadError> {
        let (header, mut reader) = header::read_header(ub_reader)?;

        for i in 0..header.number_of_threads {
            let thread_header = maybe_read_with_context!(
                thread::read_thread_header(&mut reader),
                "Reading thread {} of {}",
                i,
                header.number_of_threads,
            )?;
            eprintln!("TH: {:?}", thread_header);
            let stitches = maybe_read_with_context!(
                thread::read_stitches(&mut reader, &thread_header),
                "Reading thread {} of {}",
                i,
                header.number_of_threads,
            )?;
            eprintln!("ST Done: {}", stitches.len())
        }

        eprintln!("{:?}", header);
        unimplemented!();
    }
}
