use formats::errors::{Error, ErrorKind, Result};
use palette::Lch;
use pattern::{Color, Pattern};
use std::io::Write;
use svgtypes::{Path, WriteBuffer, WriteOptions};

const LINE_WIDTH: f64 = 0.35;
const STITCH_DIAMETER: f64 = 0.5;

fn generate_color(idx: usize, total: usize) -> Color {
    return Lch::new(50., 100., (idx as f32) * 360.0 / (total as f32)).into();
}

fn write_pattern(pattern: Pattern, writer: &mut Write) -> Result<()> {
    let (minx, miny, maxx, maxy) = pattern.get_bounds();
    let width = maxx - minx;
    let height = maxy - miny;

    writeln!(writer, "<svg version=\"1.1\"")?;
    writeln!(writer, " baseProfile=\"full\"")?;
    writeln!(writer, " xmlns=\"http://www.w3.orpg/2000/svg\"")?;
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
    let mut used_random_colors: usize = 0;
    let opt = WriteOptions {
        remove_leading_zero: true,
        use_compact_path_notation: true,
        join_arc_to_flags: true,
        ..WriteOptions::default()
    };

    for cg in pattern.color_groups {
        writeln!(writer, "    <g")?;
        // TODO: Write out stitch metadata.
        let color: Color = if let Some(thread) = cg.thread {
            thread.color
        } else {
            used_random_colors += 1;
            generate_color(used_random_colors - 1, total_colors).into()
        };
        writeln!(writer, "     stroke={}", color)?;

        assert_eq!(path.with_write_opt(&opt).to_string(), path_str);

        writeln!(writer, "     ")?;
    }

    writeln!(writer, "</svg>")?;
    Ok(())
}
