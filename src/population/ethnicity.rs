#[derive(Debug)]
pub struct Ethnicity {
    pub name: String,
}

impl Ethnicity {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
