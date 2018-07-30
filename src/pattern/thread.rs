use scarlet::color::Color;
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
    ($value:expr, $shift:expr, $mask:expr) => {
        (($value >> $shift) % (1 << $mask)) as u8
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct Thread {
    pub color: Color,
    pub name: String,
    pub code: String,
}
