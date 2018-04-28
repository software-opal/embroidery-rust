use bigdecimal::BigDecimal;

use pattern::stitch::StitchGroup;

pub enum PatternAttribute {
    Title(String),
    StitchCount(u32),
    ColorChangeCount(u32),
    BoundsMinX(BigDecimal),
}

pub struct Pattern {
    pub name: String,
    pub attributes: Vec<PatternAttribute>,
    pub stitch_groups: Vec<StitchGroup>,
}
