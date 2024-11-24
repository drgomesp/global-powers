use crate::population::Group;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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

pub struct Country<'a> {
    name: String,
    states: HashMap<String, State<'a>>,
    pub population: u64,
}

impl<'a> Country<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            states: HashMap::new(),
            population: 0,
        }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.population += state.population;
        self.states.insert(state.id.clone(), state);
    }
}

impl<'a> Debug for Country<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
            "Population", "State", "Profession", "Ethnicity", "Religion"
        )
        .expect("TODO");

        for state in self.states.values() {
            for group in state.groups.iter() {
                for sub in group.sub_groups.iter() {
                    writeln!(
                        f,
                        "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
                        sub.population,
                        state.name,
                        group.profession.name,
                        sub.ethnicity.name,
                        sub.religion.name
                    )
                    .expect("TODO");
                }
            }
        }

        write!(f, "")
    }
}
