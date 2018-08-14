use pattern::Color;
use svgtypes;
pub type SvgColor = svgtypes::Color;

impl<'a> From<&'a Color> for svgtypes::Color {
    fn from(color: &Color) -> svgtypes::Color {
        svgtypes::Color::new(color.red, color.green, color.blue)
    }
}

impl<'a> From<&'a svgtypes::Color> for Color {
    fn from(color: &svgtypes::Color) -> Color {
        Color {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }
}
