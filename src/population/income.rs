#[derive(Debug, PartialEq)]
pub enum Periodicity {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, PartialEq)]
pub struct Income {
    pub periodicity: Periodicity,

    /// Earned income includes wages, salary, tips and commissions.
    pub earned: u64,

    /// Passive income from rentals, royalties and such.
    pub passive: u64,
}

impl Income {
    pub fn new(periodicity: Periodicity, earned: u64, passive: u64) -> Self {
        Self {
            periodicity,
            earned,
            passive,
        }
    }
}
