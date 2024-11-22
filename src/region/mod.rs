use crate::population::Group;

#[derive(Debug)]
pub struct State {
    pub id: String,
    name: String,
    groups: Vec<Group>,
    population: u64,
}

impl State {
    pub fn new(id: String ,name: String) -> Self {
        Self { id, name, groups: vec![], population: 0 }
    }

    pub fn add_group(&mut self, group: Group) {
        self.population += group.population;
        self.groups.push(group);
    }
}

#[derive(Debug)]
pub struct Country {
    states: Vec<State>,
    population: u64,
}

impl Country {
    pub fn new() -> Self {
        Self { states: vec![], population: 0 }
    }

    pub fn add_state(&mut self, state: State) {
        self.population += state.population;
        self.states.push(state);
    }
}