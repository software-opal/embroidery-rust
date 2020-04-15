/*
A stitch group comprises of zero or more stitches, a optional thread and a trim flag.

A stitch represents a location where the needle enters and leaves the fabric.
The `thread` represents the thread to be used, if missing then no thread is dictated.
A trim flag indicates to add a trim command at the end of the group.

A stitch group comprising of one stitch is pretty much pointless.

A stitch represents the x,y coordinates in millimeters.
*/

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::colors::Color;
use crate::transforms::{RemoveDuplicateStitches, SplitLongStitches};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Thread {
    pub color: Color,
    pub name: String,
    pub code: String,
    pub manufacturer: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

impl Thread {
    #[inline]
    pub fn new(color: Color, name: String, code: String) -> Self {
        Self {
            color,
            name,
            code,
            manufacturer: None,
            attributes: BTreeMap::new(),
        }
    }
    #[inline]
    pub fn new_str(color: Color, name: &impl ToString, code: &impl ToString) -> Self {
        Self::new(color, name.to_string(), code.to_string())
    }
}

/// Represents mm from an arbitrary (0, 0) where positive values move up and right
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
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Display for Stitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl RemoveDuplicateStitches for ColorGroup {
    fn remove_duplicate_stitches(self) -> Self {
        ColorGroup {
            stitch_groups: self
                .stitch_groups
                .into_iter()
                .map(|cg| cg.remove_duplicate_stitches())
                .collect(),
            ..self
        }
    }
}
impl SplitLongStitches for ColorGroup {
    fn split_stitches(self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        ColorGroup {
            stitch_groups: self
                .stitch_groups
                .into_iter()
                .map(|cg| cg.split_stitches(min_x, max_x, min_y, max_y))
                .collect(),
            ..self
        }
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
    pub fn with_trim(self, trim: bool) -> Self {
        Self { trim, ..self }
    }
    pub fn with_cut(self, cut: bool) -> Self {
        Self { cut, ..self }
    }
    pub fn new(stitches: Vec<Stitch>) -> Self {
        Self {
            stitches,
            trim: false,
            cut: false,
        }
    }
    #[inline]
    pub fn iter_stitches(self: &Self) -> impl Iterator<Item = &Stitch> {
        self.stitches.iter()
    }
}

impl RemoveDuplicateStitches for StitchGroup {
    fn remove_duplicate_stitches(self) -> Self {
        let mut stitches = Vec::with_capacity(self.stitches.len());
        if self.stitches.is_empty() {
            self
        } else {
            let mut stitch_iter = self.stitches.into_iter();
            // Unchecked unwrap as we've already checked that the list isn't empty
            let mut curr_stitch = stitch_iter.next().unwrap();
            stitches.push(curr_stitch);
            for stitch in stitch_iter {
                if stitch != curr_stitch {
                    stitches.push(stitch);
                    curr_stitch = stitch;
                }
            }
            StitchGroup { stitches, ..self }
        }
    }
}
impl SplitLongStitches for StitchGroup {
    #[allow(clippy::float_cmp)]
    fn split_stitches(self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        // This function does not behave well when the values it is working with are close to the
        //  limits of floating point numbers.
        assert!(
            min_x < 0.0 && min_y < 0.0 && max_x > 0.0 && max_y > 0.0,
            "Bounds are not valid {:?}",
            (min_x, max_x, min_y, max_y)
        );

        let mut stitches = Vec::with_capacity(self.stitches.len());
        if self.stitches.is_empty() {
            self
        } else {
            let mut stitch_iter = self.stitches.into_iter().enumerate();
            // Unchecked unwrap as we've already checked that the list isn't empty
            let (_, mut curr_stitch) = stitch_iter.next().unwrap();
            stitches.push(curr_stitch);
            for (i, stitch) in stitch_iter {
                let (dx, dy) = stitch.relative_to(&curr_stitch);
                if dx < min_x || dx > max_x || dy < min_y || dy > max_y {
                    let segments_x = if dx < min_x {
                        dx / min_x
                    } else if dx > max_x {
                        dx / max_x
                    } else {
                        1.0
                    };
                    let segments_y = if dy < min_y {
                        dy / min_y
                    } else if dy > max_y {
                        dy / max_y
                    } else {
                        1.0
                    };

                    let segments = f64::ceil(f64::max(f64::abs(segments_x), f64::abs(segments_y)));
                    assert!(
                        segments > 1.0,
                        "Invalid segment count {}, {:?}. Stitch {}: {} to {}; size {:?}; bounds {:?}",
                        segments,
                        (segments_x, segments_y),
                        i,
                        curr_stitch,
                        stitch,
                        (dx, dy),
                        (min_x, max_x, min_y, max_y)
                    );
                    let Stitch { x, y } = curr_stitch;
                    let move_x = dx / segments;
                    let move_y = dy / segments;
                    assert!(
                        (segments as i32) as f64 == segments,
                        "Possible loss of precision during conversion to i32({}_f64 | {}_i32).Stitch {}: {} to {}; size {:?}; bounds {:?}",
                         segments, segments as i32, i, curr_stitch, stitch, (dx, dy), (min_x, max_x, min_y, max_y));
                    let mut partial_curr_stitch = curr_stitch;
                    for j in 1..(segments as i32) {
                        let s = Stitch::new(x + (move_x * (j as f64)), y + (move_y * (j as f64)));
                        let (s_dx, s_dy) = s.relative_to(&partial_curr_stitch);
                        assert!(
                            (min_x <= s_dx && s_dx <= max_x) && (min_y <= s_dy && s_dy <= max_y),
                            "Unable to make the stitches fit into the given bound. Stitch {}: {} to {}(size {:?}, {:?}); segment {}/{}{:?}: {} to {}; size {:?}; bounds {:?}",
                            i,
                            curr_stitch,
                            stitch,
                            (dx, dy),
                            (move_x, move_y),
                            j,
                            segments,
                            (segments_x, segments_y),
                            partial_curr_stitch,
                            s,
                            (s_dx, s_dy),
                            (min_x, max_x, min_y, max_y)
                        );
                        partial_curr_stitch = s;
                        stitches.push(s);
                    }
                    let (s_dx, s_dy) = stitch.relative_to(&partial_curr_stitch);
                    assert!(
                        (min_x <= s_dx && s_dx <= max_x) && (min_y <= s_dy && s_dy <= max_y),
                        "Unable to make the stitches fit into the given bound. Stitch {}: {} to {}(size {:?}, {:?}); segment last/{}{:?}: {} to {}; size {:?}; bounds {:?}",
                        i,
                        curr_stitch,
                        stitch,
                        (dx, dy),
                        (move_x, move_y),
                        segments,
                        (segments_x, segments_y),
                        partial_curr_stitch,
                        stitch,
                        (s_dx, s_dy),
                        (min_x, max_x, min_y, max_y)
                    );
                }
                stitches.push(stitch);
                curr_stitch = stitch;
            }
            StitchGroup { stitches, ..self }
        }
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

    #[test]
    fn split_stitches_negative() {
        let s = StitchGroup::new(vec![
            Stitch::new(0.0, 0.0),
            Stitch::new(10.0, 10.0),
            Stitch::new(-10.0, -10.0),
        ]);
        let s = s.split_stitches(-10.0, 10.0, -10.0, 10.0);
        assert_eq!(
            s.stitches,
            vec![
                Stitch::new(0.0, 0.0),
                Stitch::new(10.0, 10.0),
                Stitch::new(0.0, 0.0),
                Stitch::new(-10.0, -10.0),
            ]
        );
    }
    #[test]
    fn split_stitches_large_jump() {
        let s = StitchGroup::new(vec![Stitch::new(0.0, 0.0), Stitch::new(50.0, -50.0)]);
        let s = s.split_stitches(-10.0, 10.0, -10.0, 10.0);
        assert_eq!(
            s.stitches,
            vec![
                Stitch::new(0.0, 0.0),
                Stitch::new(10.0, -10.0),
                Stitch::new(20.0, -20.0),
                Stitch::new(30.0, -30.0),
                Stitch::new(40.0, -40.0),
                Stitch::new(50.0, -50.0),
            ]
        );
    }
    #[test]
    fn split_stitches_asymmetric_bounds() {
        let s = StitchGroup::new(vec![Stitch::new(0.0, 0.0), Stitch::new(50.0, -50.0)]);
        let s = s.split_stitches(-1.0, 10.0, -10.0, 1.0);
        assert_eq!(
            s.stitches,
            vec![
                Stitch::new(0.0, 0.0),
                Stitch::new(10.0, -10.0),
                Stitch::new(20.0, -20.0),
                Stitch::new(30.0, -30.0),
                Stitch::new(40.0, -40.0),
                Stitch::new(50.0, -50.0),
            ]
        );
    }
    #[test]
    fn split_stitches_positive() {
        let s = StitchGroup::new(vec![
            Stitch::new(0.0, 0.0),
            Stitch::new(20.0, 20.0),
            Stitch::new(4.0, 10.0),
            Stitch::new(0.0, 0.0),
        ]);
        let s = s.split_stitches(-10.0, 10.0, -10.0, 10.0);
        assert_eq!(
            s.stitches,
            vec![
                Stitch::new(0.0, 0.0),
                Stitch::new(10.0, 10.0),
                Stitch::new(20.0, 20.0),
                Stitch::new(12.0, 15.0),
                Stitch::new(4.0, 10.0),
                Stitch::new(0.0, 0.0),
            ]
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;

    use proptest::prelude::*;
    use std::f64;

    const STITCH_MAX: f64 = f64::MAX;

    prop_compose! {
            fn stitch_strategy()
                              (x in -STITCH_MAX..STITCH_MAX, y in -STITCH_MAX..STITCH_MAX)
                              -> Stitch {
                Stitch {x, y}
            }
    }
    prop_compose! {
        fn stitch_group_strategy(max_length: usize)
                                (
                                    stitches in prop::collection::vec(stitch_strategy(), 0..max_length),
                                    cut: bool,
                                    trim: bool
                                )
                                -> StitchGroup {
            StitchGroup {
                stitches, cut, trim
            }
        }
    }
    proptest! {
        #[test]
        fn split_stitches_proptest(
            sg in stitch_group_strategy(100),
            min_x in -STITCH_MAX..0.0,
            max_x in 0.0..STITCH_MAX,
            min_y in -STITCH_MAX..0.0,
            max_y in 0.0..STITCH_MAX
        ) {
            prop_assume!(min_x < 0.0 && min_y < 0.0 && max_x > 0.0 && max_y > 0.0);
            let new_sg = sg.clone().split_stitches(min_x, max_x, min_y, max_y);
            prop_assert!(new_sg.stitches.len() >= sg.stitches.len())
        }
    }
}
