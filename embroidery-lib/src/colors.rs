use palette;
use std::fmt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<Color> for palette::Srgb {
    fn from(color: Color) -> Self {
        Self::new(
            f32::from(color.red) / 255.,
            f32::from(color.green) / 255.,
            f32::from(color.blue) / 255.,
        )
    }
}

impl From<palette::Srgb> for Color {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn from(color: palette::Srgb) -> Self {
        Self {
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
