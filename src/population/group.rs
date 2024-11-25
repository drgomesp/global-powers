use crate::population::{Ethnicity, Profession, Religion};

#[derive(Debug)]
pub struct Group<'a> {
    pub profession: &'a Profession,
    pub sub_groups: Vec<SubGroup<'a>>,
    pub population: u64,
}

impl<'a> Group<'a> {
    pub fn new(profession: &'a Profession) -> Self {
        Self {
            profession,
            sub_groups: Vec::new(),
            population: 0,
        }
    }

    pub fn add_sub_group(&mut self, sub_group: SubGroup<'a>) {
        self.population += sub_group.population;
        self.sub_groups.push(sub_group)
    }

    pub fn update_population(&mut self, growth_rate: f64) {
        self.population += (self.population as f64 * (growth_rate / 100.0)) as u64;
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
