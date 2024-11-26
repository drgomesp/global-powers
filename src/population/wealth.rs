use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Wealth {
    pub level: u64,
    pub amount: u64,
}

impl Wealth {
    pub fn new(level: u64, amount: u64) -> Self {
        Self { level, amount }
    }
}

impl std::fmt::Display for Wealth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{0: <10} | {1: <20}", "Level", "Amount")?;
        writeln!(f, "{:?} | {:?}", self.level, self.amount)
    }
}
