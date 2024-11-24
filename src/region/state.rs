use crate::population::group::Group;
use crate::region::Region;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct State<'a> {
    pub id: String,
    pub name: String,
    region: Region,
    pub groups: BTreeMap<u64, Group<'a>>,
    pub population: u64,
    pub population_percentage: f64,
}

impl<'a> State<'a> {
    pub fn new(id: String, name: String, region: Region, population_percentage: f64) -> Self {
        Self {
            id,
            name,
            region,
            groups: BTreeMap::new(),
            population: 0,
            population_percentage,
        }
    }

    pub fn add_group(&mut self, group: Group<'a>) {
        self.population += group.population;
        self.groups.insert(group.population, group);
    }
}
