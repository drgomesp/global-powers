#[derive(Debug)]
pub struct Ethnicity {
    pub name: String,
    pub population_percentage: f64,
}

impl Ethnicity {
    pub fn new(name: String, percentage: f64) -> Self {
        Self {
            name,
            population_percentage: percentage,
        }
    }
}
