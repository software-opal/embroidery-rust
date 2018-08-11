use palette;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for palette::Srgb {
    fn from(color: Color) -> palette::Srgb {
        palette::Srgb::new(
            (color.red as f32) / 255.,
            (color.green as f32) / 255.,
            (color.blue as f32) / 255.,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}", self.red)
    }
}
