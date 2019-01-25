use std::{f64, iter::Iterator};

use crate::stitch::{ColorGroup, Stitch};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PatternAttribute {
    Arbitary(String, String),
    Title(String),
    Author(String),
    Copyright(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub name: String,
    pub attributes: Vec<PatternAttribute>,
    pub color_groups: Vec<ColorGroup>,
}

impl Pattern {
    pub fn iter_stitches(self: &Self) -> impl Iterator<Item = &Stitch> {
        self.color_groups.iter().flat_map(|g| g.iter_stitches())
    }

    pub fn get_bounds(self: &Self) -> (f64, f64, f64, f64) {
        let mut minx: f64 = f64::NAN;
        let mut miny: f64 = f64::NAN;
        let mut maxx: f64 = f64::NAN;
        let mut maxy: f64 = f64::NAN;
        for stitch in self.iter_stitches() {
            minx = minx.min(stitch.x);
            miny = miny.min(stitch.y);
            maxx = maxx.max(stitch.x);
            maxy = maxy.max(stitch.y);
        }
        if minx.is_nan() || miny.is_nan() || maxx.is_nan() || maxy.is_nan() {
            (0., 0., 0., 0.)
        } else {
            (minx, miny, maxx, maxy)
        }
    }
}
