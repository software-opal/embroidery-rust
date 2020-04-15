mod error;
mod formats;

#[macro_use]
pub extern crate failure;
#[macro_use]
pub extern crate log;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Seek;
use std::path::Path;

use simplelog::*;

use embroidery_lib::prelude::{ReadError, WriteError};

use crate::error::Error;
use crate::formats::get_all;
use std::env;

fn main() -> Result<(), Error> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
    )?;

    let loader_unloaders = get_all();

    for file in env::args().skip(1) {
        let path = Path::new(&file);
        let file_name = path.file_name().ok_or("Path must have an filename")?.to_string_lossy();

        let mut loader_result = None;
        {
            let mut reader = BufReader::new(File::open(file.clone())?);
            for (i, format) in loader_unloaders.iter().enumerate() {
                reader.seek(std::io::SeekFrom::Start(0))?;
                if let Some(loader) = format.reader() {
                    match loader.read_pattern(&mut reader) {
                        Ok(p) => {
                            loader_result = Some((i, p));
                            break;
                        },
                        Err(ReadError::InvalidFormat(msg, ctx)) => warn!(
                            "Loader {} cannot parse file {}. Reason: {}\n Context: {:#?}",
                            format.name(),
                            file_name,
                            msg,
                            ctx
                        ),
                        Err(err) => return Err(err.into()),
                    }
                }
            }
        }
        let (loader_idx, pattern) =
            loader_result.ok_or_else(|| format!("The path cannot be read by any of the loaders: {}", file_name))?;

        for (i, format) in loader_unloaders.iter().enumerate() {
            if i == loader_idx {
                continue;
            }
            let extensions = format.extensions();
            assert!(!extensions.is_empty());
            if let Some(writer) = format.writer() {
                let ext = extensions[0];
                let output = path.with_file_name(format!("{}.{}", file_name, ext));
                let mut out = BufWriter::new(File::create(output)?);
                match writer.write_pattern(&pattern, &mut out) {
                    Ok(()) => {},
                    Err(WriteError::UnsupportedStitch {
                        stitch,
                        idx: maybe_idx,
                        ctx: _,
                    }) => {
                        let idx = match maybe_idx {
                            Some(idx) => format!("{}", idx),
                            None => ("unknown").to_string(),
                        };
                        return Err(format!(
                            "Writer {} cannot write one of the stitches {:?}(at {}): {}",
                            ext, stitch, idx, file_name,
                        )
                        .into());
                    },
                    Err(WriteError::Std(err, _)) => return Err(err.into()),
                }
            }
        }
    }
    Ok(())
}
