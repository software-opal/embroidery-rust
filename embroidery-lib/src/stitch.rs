/*
A stitch group comprises of zero or more stitches, a optional thread and a trim flag.

A stitch represents a location where the needle enters and leaves the fabric.
The `thread` represents the thread to be used, if missing then no thread is dictated.
A trim flag indicates to add a trim command at the end of the group.

A stitch group comprising of one stitch is pretty much pointless.

A stitch represents the x,y coordinates in millimeters.
*/

use crate::colors::Color;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Thread {
    pub color: Color,
    pub name: String,
    pub code: String,
}

impl Thread {
    #[inline]
    pub const fn new(color: Color, name: String, code: String) -> Self {
        Self { color, name, code }
    }
    #[inline]
    pub fn new_str(color: Color, name: &impl ToString, code: &impl ToString) -> Self {
        Self {
            color,
            name: name.to_string(),
            code: code.to_string(),
        }
    }
}

/// Represents mm from an arbitary (0, 0) where positive values move up and right
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stitch {
    pub x: f64,
    pub y: f64,
}

impl Stitch {
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn relative_to(&self, other: &Self) -> (f64, f64) {
        (self.x - other.x, self.y - other.y)
    }
    #[inline]
    pub fn distance_to(&self, other: &Self) -> f64 {
        let (dx, dy) = self.relative_to(other);
        ((dx * dx) + (dy * dy)).sqrt()
    }
    #[inline]
    pub fn zero() -> Self {
        Self::default()
    }
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorGroup {
    pub thread: Option<Thread>,
    pub stitch_groups: Vec<StitchGroup>,
}

impl ColorGroup {
    #[inline]
    pub fn iter_stitches(self: &Self) -> impl Iterator<Item = &Stitch> {
        self.stitch_groups.iter().flat_map(StitchGroup::iter_stitches)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct StitchGroup {
    pub stitches: Vec<Stitch>,
    pub trim: bool,
    pub cut: bool,
}

impl StitchGroup {
    #[inline]
    pub fn iter_stitches(self: &Self) -> impl Iterator<Item = &Stitch> {
        self.stitches.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stitch_relative_to() {
        let s = Stitch { x: 1.0, y: 1.0 };
        assert_eq!(s.relative_to(&Stitch::zero()), (1.0, 1.0));
        assert_eq!(Stitch::zero().relative_to(&s), (-1.0, -1.0));
    }

    #[test]
    fn stitch_distance_to() {
        let s = Stitch { x: 3.0, y: 4.0 };
        assert_eq!(s.distance_to(&Stitch::zero()), 5.0);
        assert_eq!(Stitch::zero().distance_to(&s), 5.0);
    }
}
