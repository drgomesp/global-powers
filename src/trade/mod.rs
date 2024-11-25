#[derive(Debug, PartialEq)]
pub enum ConsumptionLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, PartialEq)]
pub struct Good {
    pub name: String,
}
