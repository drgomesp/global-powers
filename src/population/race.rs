#[derive(Debug)]
pub struct Race {
    pub name: String,
}

impl Race {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
