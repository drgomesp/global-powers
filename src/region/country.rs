use crate::region::state::State;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

pub struct Country<'a> {
    pub name: String,
    states: BTreeMap<u64, State<'a>>,
    pub population: u64,
}

impl<'a> Country<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            states: BTreeMap::new(),
            population: 0,
        }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.population += state.population;
        self.states.insert(state.population, state);
    }
}

impl<'a> Debug for Country<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name).expect("TODO");

        writeln!(
            f,
            "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
            "Population", "State", "Profession", "Ethnicity", "Religion"
        )
        .expect("TODO");

        for state in self.states.values() {
            for (_, group) in state.groups.iter() {
                for (sub_group_population, sub_group) in group.sub_groups.iter().rev() {
                    writeln!(
                        f,
                        "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
                        sub_group_population,
                        state.name,
                        group.profession.name,
                        sub_group.ethnicity.name,
                        sub_group.religion.name
                    )
                    .expect("TODO");
                }
            }
        }

        write!(f, "")
    }
}
