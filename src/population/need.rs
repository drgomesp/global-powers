use crate::trade::{ConsumptionLevel, ConsumptionTendency, Good};

#[derive(Clone, Debug, PartialEq)]
pub struct Need {
    good: Good,
    consumption_level: ConsumptionLevel,
    consumption_tendency: ConsumptionTendency,
}
