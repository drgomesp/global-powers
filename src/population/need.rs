use crate::trade::{ConsumptionLevel, Good};

#[derive(Debug, Default, PartialEq)]
pub struct Need {
    goods: Vec<(Good, ConsumptionLevel)>,
}

impl Need {
    pub fn new() -> Self {
        Self { goods: Vec::new() }
    }
}
