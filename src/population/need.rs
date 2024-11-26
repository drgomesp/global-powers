use crate::trade::{ConsumptionLevel, ConsumptionTendency, Good};

#[derive(Debug, PartialEq)]
pub struct Need {
    good: Good,
    consumption_level: ConsumptionLevel,
    consumption_over_time: ConsumptionTendency,
}
