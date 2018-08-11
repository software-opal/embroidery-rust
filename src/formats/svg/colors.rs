use pattern::Color;
use svgtypes;
pub type SvgColor = svgtypes::Color;

impl From<Color> for svgtypes::Color {
    fn from(color: Color) -> svgtypes::Color {
        svgtypes::Color::new(color.red, color.green, color.blue)
    }
}

impl From<svgtypes::Color> for Color {
    fn from(color: Color) -> svgtypes::Color {
        svgtypes::Color::new(color.red, color.green, color.blue)
    }
}
