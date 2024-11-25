use crate::population::StandardOfLiving;
use crate::trade::{ConsumptionLevel, Good};
use std::fmt::Formatter;

#[derive(Debug, Default, PartialEq)]
pub struct Needs {
    pub goods: Vec<(Good, ConsumptionLevel)>,
}

#[derive(Debug, PartialEq)]
pub struct Wealth {
    pub level: u64,
    pub amount: u64,
    pub sol: StandardOfLiving,
}

impl Wealth {
    pub fn new(level: u64, amount: u64, sol: StandardOfLiving) -> Self {
        Self { level, amount, sol }
    }
}

impl std::fmt::Display for Wealth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{0: <10} | {1: <20} | {2: <20}",
            "Level", "Amount", "Standard of Living"
        )?;

        writeln!(f, "{:?} | {:?} | {:?}", self.level, self.amount, self.sol,)
    }
}
