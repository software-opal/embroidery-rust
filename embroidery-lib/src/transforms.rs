pub trait RemoveDuplicateStitches {
    fn remove_duplicate_stitches(self) -> Self;
}

pub trait SplitLongStitches {
    fn split_stitches(self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self;
}
