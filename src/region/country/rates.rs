#[derive(Debug)]
pub struct Rates {
    pub population_growth: f64,
}
impl Rates {
    pub fn new(population_growth: f64) -> Self {
        Self { population_growth }
    }
}
