macro_rules! expand_from_delta {
    ($x:ident, $y:ident,y, $sign:tt, $val:tt) => {
        $y = $y $sign $val
    };
    ($x:ident, $y:ident,x, $sign:tt, $val:tt) => {
        $x = $x $sign $val
    };
}

macro_rules! expand_to_condition {
    ($x:ident, $y:ident,y, $sign:tt, $val:tt: $block:block) => {
        println!("  y={}, {}", $y, stringify!($sign:tt, $val:tt: $block:block));
        if (0 $sign $y) >= (($val - 1) / 2 + 1) {
            $block;
            println!(">>y={}, {}", $y, stringify!($sign:tt, $val:tt: $block:block));
            $y = $y - (0 $sign $val);
        }
    };
    ($x:ident, $y:ident,x, $sign:tt, $val:tt: $block:block) => {
        println!("  x={}, {}", $x, stringify!($sign:tt, $val:tt: $block:block));
        if (0 $sign $x) >= (($val - 1) / 2 + 1) {
            $block;
            println!(">>x={}, {}", $x, stringify!($sign:tt, $val:tt: $block:block));
            $x = $x - (0 $sign $val);
        }
    };
}

macro_rules! stitch_definitions {
    ($($bits:tt -> ($var: ident $sign:tt $val:tt)),+) => {
        #[inline]
        fn from_int(val: u32) -> (i8, i8) {
            let mut x: i8 = 0;
            let mut y: i8 = 0;
            $(
                if val & $bits == $bits {
                    expand_from_delta!(x, y, $var, $sign, $val);
                }
            )+
            return (x, y)
        }

        #[inline]
        fn to_int(_x: i8, _y: i8) -> Option<u32> {
            if _x < -121 || _x > 121 || _y < -121 || _y > 121 {
                return None;
            }
            let mut x = _x;
            let mut y = _y;
            let mut bytes: u32 = 0_00_00_03;
            $(
                println!("{}, {}", x, y);
                expand_to_condition!(x, y, $var, $sign, $val: {
                    bytes |= $bits;
                });
            )+
            if x != 0 || y != 0 {
                None
            } else {
                Some(bytes)
            }
        }
    };
}

stitch_definitions!(
    0x_00_00_20 -> (y + 81),
    0x_00_00_10 -> (y - 81),
    0x_00_00_08 -> (x - 81),
    0x_00_00_04 -> (x + 81),

    0x_00_20_00 -> (y + 27),
    0x_00_10_00 -> (y - 27),
    0x_00_08_00 -> (x - 27),
    0x_00_04_00 -> (x + 27),

    0x_20_00_00 -> (y + 9),
    0x_10_00_00 -> (y - 9),
    0x_08_00_00 -> (x - 9),
    0x_04_00_00 -> (x + 9),

    0x_00_80_00 -> (y + 3),
    0x_00_40_00 -> (y - 3),
    0x_00_02_00 -> (x - 3),
    0x_00_01_00 -> (x + 3),

    0x_80_00_00 -> (y + 1),
    0x_40_00_00 -> (y - 1),
    0x_02_00_00 -> (x - 1),
    0x_01_00_00 -> (x + 1)
);

pub enum StitchInformation {
    Move(i8, i8),
    Jump(i8, i8),
    ColorChange(i8, i8),
    End,
}

impl StitchInformation {
    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        let val: u32 = bytes[2] as u32;
        let val: u32 = (bytes[1] as u32) | (val << 8);
        let (x, y) = from_int(val);
        if val & 0x_00_00_80 == 0x_00_00_80 {
            return StitchInformation::Jump(x, y);
        } else if val & 0x_00_00_40 == 0x_00_00_40 {
            return StitchInformation::ColorChange(x, y);
        } else if val == 0x_00_00_F3 {
            return StitchInformation::End;
        } else {
            return StitchInformation::Move(x, y);
        }
    }
    pub fn to_bytes(us: Self) -> Option<u32> {
        let matched = match us {
            StitchInformation::Move(x, y) => to_int(x, y),
            StitchInformation::Jump(x, y) => to_int(x, y),
            StitchInformation::ColorChange(x, y) => to_int(x, y),
            StitchInformation::End => Some(0x00_00_F3),
        };
        match matched {
            None => None,
            Some(bytes) => match us {
                StitchInformation::Jump(_, _) => Some(bytes | 0x00_00_80),
                StitchInformation::ColorChange(_, _) => Some(bytes | 0x00_00_40),
                _ => Some(bytes),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_gen_from_int() {
        assert_eq!(from_int(0x00_00_00), (0, 0));
        assert_eq!(from_int(0x00_00_20), (0, 81));
        assert_eq!(from_int(0x00_00_18), (-81, -81));
    }

    #[test]
    fn test_macro_gen_to_int() {
        assert_eq!(to_int(0, 0), Some(0x00_00_03));
        assert_eq!(to_int(127, 127), None);
        assert_eq!(to_int(127, 5), None);
        assert_eq!(to_int(-121, -121), Some(0x5A_5A_1B));
        assert_eq!(to_int(121, -121), Some(0x55_55_17));
        assert_eq!(to_int(-12, 64), Some(0xA8_12_23));
        assert_eq!(to_int(8, -8), Some(0x96_00_03));
    }

    #[test]
    fn test_macro_gen_valids_roundtrip() {
        for x in -121..121 {
            for y in -121..121 {
                let test_val = to_int(x, y).unwrap();
                let (test_x, test_y) = from_int(test_val);
                assert_eq!(
                    (test_x, test_y),
                    (x, y),
                    "({}, {}) -> {:06X} -> ({}, {})",
                    x,
                    y,
                    test_val,
                    test_x,
                    test_y
                )
            }
        }
    }
}
