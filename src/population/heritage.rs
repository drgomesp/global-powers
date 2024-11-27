#[derive(Debug)]
pub struct Heritage {
    pub name: String,
}

impl Heritage {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
