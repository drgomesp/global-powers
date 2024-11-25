use crate::population::group::Group;
use crate::region::Region;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State<'a> {
    pub id: String,
    pub name: String,
    pub region: Region,
    pub groups: HashMap<String, Group<'a>>,
    pub population_percentage: f64,
}

impl<'a> State<'a> {
    pub fn new(id: String, name: String, region: Region, population_percentage: f64) -> Self {
        Self {
            id,
            name,
            region,
            groups: HashMap::new(),
            population_percentage,
        }
    }

    pub fn add_group(&mut self, group: Group<'a>) {
        self.groups.insert(group.profession.name.clone(), group);
    }

    pub fn update_population(&mut self, growth_rate: f64) {
        for (_, group) in self.groups.iter_mut() {
            group.update_population(growth_rate);
        }
    }

    pub fn population(&self) -> u64 {
        self.groups.iter().map(|(_, group)| group.population).sum()
    }
}
