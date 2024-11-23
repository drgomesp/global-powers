use strum_macros::EnumIter;

#[derive(Debug)]
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
pub struct Ethnicity {
    name: String,
}

impl Ethnicity {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    state_id: String,
    profession: &'a Profession,
    sub_groups: Vec<SubGroup<'a>>,
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
    ethnicity: &'a Ethnicity,
    religion: &'a Religion,
    population: u64,
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
