use crate::region::State;

#[derive(Debug)]
pub enum Class {
    Upper,
    Middle,
    Lower,
}

#[derive(Debug)]
pub struct Profession {
    class: Class,
    name: String,
}

impl Profession {
    pub fn new(class: Class, name: String) -> Self {
        Self { class, name }
    }
}

#[derive(Debug)]
pub struct Religion {
    name: String,
}

impl Religion {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct Group {
    state_id: String,
    profession: Profession,
    religion: Religion,
    pub population: u64,
}

impl Group {
    pub fn new(state_id: String, profession: Profession, religion: Religion, population: u64) -> Self {
        Self {
            state_id,
            profession,
            religion,
            population,
        }
    }
}
