use strum_macros::EnumIter;

#[derive(Debug, PartialEq)]
pub enum Class {
    Upper,
    Middle,
    Lower,
}

#[derive(Debug, EnumIter)]
pub enum Age {
    Children,
    Adult,
    Senior,
}

#[derive(Debug, PartialEq)]
pub struct Profession {
    class: Class,
    pub name: String,
}
impl Profession {
    pub fn new(class: Class, name: String) -> Self {
        Self { class, name }
    }
}

#[derive(Debug)]
pub struct Religion {
    pub name: String,
}

impl Religion {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct Ethnicity {
    pub name: String,
    pub percentage: f64,
}

impl Ethnicity {
    pub fn new(name: String, percentage: f64) -> Self {
        Self { name, percentage }
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    state_id: String,
    pub profession: &'a Profession,
    pub sub_groups: Vec<SubGroup<'a>>,
    pub population: u64,
}

impl<'a> Group<'a> {
    pub fn new(state_id: String, profession: &'a Profession) -> Self {
        Self {
            state_id,
            profession,
            sub_groups: Vec::new(),
            population: 0,
        }
    }

    pub fn add_sub_group(&mut self, sub_group: SubGroup<'a>) {
        self.population += sub_group.population;
        self.sub_groups.push(sub_group);
    }
}

#[derive(Debug)]
pub struct SubGroup<'a> {
    pub ethnicity: &'a Ethnicity,
    pub religion: &'a Religion,
    pub population: u64,
}

impl<'a> SubGroup<'a> {
    pub fn new(ethnicity: &'a Ethnicity, religion: &'a Religion, population: u64) -> Self {
        Self {
            ethnicity,
            religion,
            population,
        }
    }
}
