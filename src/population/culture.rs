#[derive(Clone, Debug)]
pub struct Culture {
    pub name: String,
}

impl Culture {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
