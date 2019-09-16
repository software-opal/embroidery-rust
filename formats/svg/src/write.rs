use std::io::Write;

use palette::{Lch, Srgb};
use svgtypes::{PathBuilder, WriteBuffer, WriteOptions};

use embroidery_lib::format::errors::WriteResult as Result;
use embroidery_lib::format::traits::PatternWriter;
use embroidery_lib::prelude::*;

const LINE_WIDTH: f64 = 0.2;
const STITCH_DIAMETER: f64 = 0.4;

pub struct SvgPatternWriter {}

impl Default for SvgPatternWriter {
    fn default() -> Self {
        SvgPatternWriter {}
    }
}

impl PatternWriter for SvgPatternWriter {
    fn write_pattern(&self, pattern: &Pattern, writer: &mut dyn Write) -> Result<()> {
        write_pattern(pattern, writer)
    }
}

fn generate_color(idx: usize, total: usize) -> Srgb {
    Lch::new(50., 100., (idx as f32) * 360.0 / (total as f32)).into()
}

fn write_pattern(pattern: &Pattern, writer: &mut dyn Write) -> Result<()> {
    let (min_x, min_y, max_x, max_y) = pattern.get_bounds();
    let width = max_x - min_x;
    let height = max_y - min_y;

    writeln!(writer, "<?xml version='1.0' encoding='UTF-8' standalone='no'?>")?;
    writeln!(writer, "<svg")?;
    writeln!(writer, " xmlns:svg=\"http://www.w3.org/2000/svg\"")?;
    writeln!(writer, " xmlns=\"http://www.w3.org/2000/svg\"")?;
    writeln!(writer, " version=\"1.1\"")?;
    writeln!(writer, " preserveAspectRatio=\"xMidYMid meet\"")?;
    writeln!(writer, " shape-rendering='geometricPrecision'")?;
    writeln!(writer, " text-rendering='geometricPrecision'")?;
    writeln!(writer, " image-rendering='optimizeQuality'")?;
    writeln!(writer, " width=\"{}mm\"", width + 20.)?;
    writeln!(writer, " height=\"{}mm\"", height + 20.)?;
    writeln!(
        writer,
        " viewBox=\"{} {} {} {}\"",
        min_x - 10.,
        -10.,
        width + 20.,
        height + 20.
    )?;
    writeln!(writer, ">")?;

    // TODO: Write out metadata
    // writeln!(writer, "  <metadata>")?;
    // writeln!(writer, "    <rdf:RDF>")?;
    // writeln!(writer, "      <cc:Work rdf:about=''>")?;
    // writeln!(writer, "        <dc:format>image/svg+xml</dc:format>")?;
    // writeln!(writer, "        <dc:type rdf:resource='http://purl.org/dc/dcmitype/StillImage' />")?;
    // writeln!(writer, "      </cc:Work>")?;
    // writeln!(writer, "    </rdf:RDF>")?;
    // writeln!(writer, "  </metadata>")?;

    let total_colors = pattern.color_groups.iter().filter(|cg| cg.thread == None).count();
    let mut used_random_colors: usize = 0;
    let opt = WriteOptions {
        remove_leading_zero: true,
        use_compact_path_notation: true,
        join_arc_to_flags: true,
        ..WriteOptions::default()
    };

    for cg in pattern.color_groups.iter() {
        // TODO: Write out stitch metadata.
        let color: Color = if let Some(ref thread) = cg.thread {
            // Need clone to use the color later.
            thread.color
        } else {
            used_random_colors += 1;
            generate_color(used_random_colors - 1, total_colors).into()
        };
        writeln!(writer, "    <g")?;
        writeln!(writer, "     fill='none'")?;
        writeln!(writer, "     stroke='{}'", color)?;
        writeln!(writer, "     stroke-width='{}'", LINE_WIDTH)?;
        writeln!(writer, "     stroke-linecap='round'")?;
        writeln!(writer, "     stroke-linejoin='round'")?;
        writeln!(writer, "    >")?;

        for sg in cg.stitch_groups.iter() {
            let mut path = PathBuilder::with_capacity(sg.stitches.len() + 2);
            if let Some(stitch) = sg.stitches.get(0) {
                path = path.move_to(stitch.x, max_y - stitch.y);
            }
            writeln!(writer, "      <g stroke='none' fill='{}' class='emb_ignore'>", color)?;
            for (i, stitch) in sg.stitches.iter().enumerate() {
                if i != 0 {
                    // reverse y axis so +ve y moves up
                    path = path.line_to(stitch.x, max_y - stitch.y);
                }
                writeln!(
                    writer,
                    "        <circle cx='{}' cy='{}' r='{}' />",
                    stitch.x,
                    max_y - stitch.y,
                    STITCH_DIAMETER / 2.
                )?;
            }
            writeln!(writer, "      </g>")?;
            writeln!(
                writer,
                "      <path d='{}' />",
                path.finalize().with_write_opt(&opt).to_string()
            )?;
        }
        writeln!(writer, "    </g>")?;
    }

    writeln!(writer, "</svg>")?;
    Ok(())
}
