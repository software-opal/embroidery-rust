use crate::pattern::Pattern;
use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PatternCollection {
    pub patterns: BTreeMap<String, Pattern>,
}
impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: BTreeMap::new(),
        }
    }
}

impl From<BTreeMap<String, Pattern>> for PatternCollection {
    fn from(patterns: BTreeMap<String, Pattern>) -> Self {
        Self { patterns }
    }
}

impl FromIterator<(String, Pattern)> for PatternCollection {
    fn from_iter<I: IntoIterator<Item = (String, Pattern)>>(iter: I) -> Self {
        Self {
            patterns: BTreeMap::from_iter(iter),
        }
    }
}
