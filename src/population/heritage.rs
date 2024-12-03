#[derive(Debug)]
pub struct Heritage {
    pub name: String,
    pub population_percentage: f64,
}

impl Heritage {
    pub fn new(name: String, percentage: f64) -> Self {
        Self {
            name,
            population_percentage: percentage,
        }
    }
}
