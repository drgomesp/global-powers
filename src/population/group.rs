use crate::population::income::Income;
use crate::population::wealth::Wealth;
use crate::population::{Ethnicity, Profession, Religion, StandardOfLiving};

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
        self.population = 0;

        for mut sub_group in self.sub_groups.iter_mut() {
            sub_group.population += (sub_group.population as f64 * (growth_rate / 100.0)) as u64;
            self.population += sub_group.population;
        }
    }
}

#[derive(Debug)]
pub struct SubGroup<'a> {
    pub ethnicity: &'a Ethnicity,
    pub religion: &'a Religion,
    pub wealth: Wealth,
    pub income: Income,
    pub sol: StandardOfLiving,
    pub population: u64,
}

impl<'a> SubGroup<'a> {
    pub fn new(
        ethnicity: &'a Ethnicity,
        religion: &'a Religion,
        wealth: Wealth,
        income: Income,
        sol: StandardOfLiving,
        population: u64,
    ) -> Self {
        Self {
            ethnicity,
            religion,
            wealth,
            income,
            sol,
            population,
        }
    }
}
