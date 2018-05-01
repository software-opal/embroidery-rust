use pattern::stitch::StitchGroup;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternAttribute {
    Arbitary(String, String),
    Title(String),
    Author(String),
    Copyright(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub name: String,
    pub attributes: Vec<PatternAttribute>,
    pub stitch_groups: Vec<StitchGroup>,
}
