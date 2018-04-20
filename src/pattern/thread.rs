use std::str::FromStr;

error_chain! {
  errors {
    ColorParseError(input: String) {
      description("Invalid color string")
      display("Invalid color string: '{}'", input)
    }
  }
}

macro_rules! shift_truncate {
    ($value: expr, $shift: expr, $mask: expr) => ((($value >> $shift) % (1 << $mask)) as u8);
}

pub struct Thread {
    pub color: Color,
    pub name: String,
    pub code: String,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Color {
    type Err = Error;
    fn from_str(s: &str) -> Result<Color> {
        let mut chrs = s.chars().peekable();
        if chrs.peek() == Some(&'#') {
            chrs.next();
        }
        let cleaned: String = chrs.collect();
        let length = cleaned.chars().count();
        let value: u32 = match u32::from_str_radix(&cleaned, 16) {
            Ok(v) => v,
            Err(_) => bail!(ErrorKind::ColorParseError(String::from(s))),
        };
        let (r, g, b) = match length {
            3 => (
                0x11u8 * shift_truncate!(value, 8, 4),
                0x11u8 * shift_truncate!(value, 4, 4),
                0x11u8 * shift_truncate!(value, 0, 4),
            ),
            6 => (
                shift_truncate!(value, 16, 8),
                shift_truncate!(value, 8, 8),
                shift_truncate!(value, 0, 8),
            ),
            _ => bail!(ErrorKind::ColorParseError(String::from(s))),
        };
        Ok(Color { r, g, b })
    }
}

impl Color {}
