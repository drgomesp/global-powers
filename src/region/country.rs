use crate::region::state::State;
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Country<'a> {
    pub name: String,
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
            "{} | Pop: {}",
            self.name,
            self.population.to_formatted_string(&Locale::en)
        )?;

        writeln!(
            f,
            "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
            "Population", "State", "Profession", "Ethnicity", "Religion"
        )?;

        for state in self.states.values() {
            for (_, group) in state.groups.iter() {
                let mut v: Vec<_> = group.sub_groups.iter().collect();
                v.sort_by(|a, b| a.population.partial_cmp(&b.population).unwrap());

                for sub_group in v.iter().rev() {
                    writeln!(
                        f,
                        "{0: <10} | {1: <20} | {2: <20} | {3: <10} | {4: <10}",
                        sub_group.population.to_formatted_string(&Locale::en),
                        state.name,
                        group.profession.name,
                        sub_group.ethnicity.name,
                        sub_group.religion.name
                    )?;
                }
            }
        }

        write!(f, "")
    }
}
