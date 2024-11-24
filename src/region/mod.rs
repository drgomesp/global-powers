use crate::population::Group;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State<'a> {
    pub id: String,
    name: String,
    groups: Vec<Group<'a>>,
    pub population: u64,
    pub population_percentage: f64,
}

impl<'a> State<'a> {
    pub fn new(id: String, name: String, population_percentage: f64) -> Self {
        Self {
            id,
            name,
            groups: vec![],
            population: 0,
            population_percentage,
        }
    }

    pub fn add_group(&mut self, group: Group<'a>) {
        self.population += group.population;
        self.groups.push(group);
    }
}

#[derive(Debug)]
pub struct Country<'a> {
    states: HashMap<String, State<'a>>,
    pub population: u64,
}

impl<'a> Country<'a> {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            population: 0,
        }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.population += state.population;
        self.states.insert(state.id.clone(), state);
    }

    pub fn get_state_population(&self, state_id: &str) -> Option<u64> {
        self.states.get(state_id).map(|state| state.population)
    }
}
