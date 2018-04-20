use formats::PatternLoader;

struct DstHeader {
    label: [char; 16],
    /// Stored in-file as a 7-character long decimal.
    stitch_count: u32,
    /// Stored in-file as a 3-character long decimal.
    color_count: u32,
    /// Stored in-file as a 5-character long space-padded decimal.
    max_x_bounds: u32,
    min_x_bounds: u32,
    max_y_bounds: u32,
    min_y_bounds: u32,
}

pub struct DstPatternLoader {}

impl PatternLoader for DstPatternLoader {
    fn isLoadable(&self, item: Read) -> IOResult<bool> {
        // Load the header
        // Check the last byte of the file? maybe
        return Ok(true);
    }

    fn read_pattern(&self, item: Read) -> Result<Pattern> {
        // Read the header

    }
}
