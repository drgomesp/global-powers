#[derive(Clone, Debug, PartialEq)]
pub enum ConsumptionLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConsumptionTendency {
    Temporary,
    Constant,
    Linear,
    Exponential,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Good {
    pub name: String,
}
