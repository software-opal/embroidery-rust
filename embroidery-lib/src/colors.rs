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
            f32::from(color.red) / 255.,
            f32::from(color.green) / 255.,
            f32::from(color.blue) / 255.,
        )
    }
}

impl From<palette::Srgb> for Color {
    fn from(color: palette::Srgb) -> Color {
        Color {
            red: (color.red * 255.) as u8,
            green: (color.green * 255.) as u8,
            blue: (color.blue * 255.) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
