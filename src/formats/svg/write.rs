use formats::errors::{Error, ErrorKind, Result};
use pattern::Pattern;
use std::io::Write;

const LINE_WIDTH: f64 = 0.35;
const STITCH_DIAMETER: f64 = 0.5;

fn write_pattern(pattern: Pattern, writer: &mut Write) -> Result<()> {
    let (minx, miny, maxx, maxy) = pattern.get_bounds();
    let width = maxx - minx;
    let height = maxy - miny;

    writeln!(writer, "<svg version=\"1.1\"")?;
    writeln!(writer, " baseProfile=\"full\"")?;
    writeln!(writer, " xmlns=\"http://www.w3.org/2000/svg\"")?;
    writeln!(writer, " preserveAspectRatio=\"xMidYMid meet\"")?;
    writeln!(writer, " width=\"{}mm\"", width)?;
    writeln!(writer, " height=\"{}mm\"", height)?;
    writeln!(
        writer,
        " viewBox=\"{} {} {} {}\"",
        minx, miny, width, height
    )?;
    writeln!(writer, ">")?;

    writeln!(writer, "  <metadata>")?;
    // TODO: Write out metadata
    writeln!(writer, "  </metadata>")?;

    let total_colors = pattern
        .color_groups
        .iter()
        .filter(|cg| cg.thread == None)
        .count();
    let mut used_random_colors = 0;

    for cg in pattern.color_groups {
        writeln!(writer, "    <g")?;
        // TODO: Write out stitch metadata.
        let color = if Some(thread) == cg.thread {
            thread.color
        } else {

        };
        writeln!(writer, "     ")?;
    }

    writeln!(writer, "</svg>")?;
    Ok(())
}
