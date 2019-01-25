use crate::pattern::colors::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Thread {
    pub color: Color,
    pub name: String,
    pub code: String,
}
