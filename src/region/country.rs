pub mod rates;

use crate::region::country::rates::Rates;
use crate::region::state::State;
use crate::region::Region;
use num_format::{Locale, ToFormattedString};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Country<'a> {
    pub name: String,
    states: HashMap<String, State<'a>>,
    rates: Rates,
}

impl<'a> Country<'a> {
    pub fn update_population(&mut self) {
        for (_, state) in self.states.iter_mut() {
            state.update_population(self.rates.population_growth);
        }
    }
}

impl<'a> Country<'a> {
    pub fn new(name: String, rates: Rates) -> Self {
        Self {
            name,
            states: HashMap::new(),
            rates,
        }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.states.insert(state.id.clone(), state);
    }

    pub fn get_population(&self) -> u64 {
        self.states.values().map(|state| state.population()).sum()
    }

    pub fn get_population_by_state(&self, state_id: String) -> u64 {
        self.states.get(&state_id).unwrap().population()
    }

    pub fn get_population_by_region(&self, region: Region) -> u64 {
        self.states
            .values()
            .map(|state| {
                if state.region == region {
                    state.population()
                } else {
                    0
                }
            })
            .sum()
    }
}

impl<'a> Debug for Country<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{0: <10} | {1: <10} | {2: <20} | {3: <10} | {4: <15} | {5: <10} | {6: <10} | {7: <10} | {8: <10} | {9: <10}",
            "Population",
            "State",
            "Profession",
            "Race",
            "Nationality",
            "Ethnicity",
            "Religion",
            "Wealth",
            "Income",
            "Standard of Living",
        )?;

        for state in self.states.values() {
            if !state.id.eq("SP") {
                continue;
            }

            for (_, group) in state.groups.iter() {
                let mut v: Vec<_> = group.sub_groups.iter().collect();
                v.sort_by(|a, b| a.population.partial_cmp(&b.population).unwrap());

                for sub_group in v.iter().rev() {
                    writeln!(
                        f,
                        "{0: <10} | {1: <10} | {2: <20} | {3: <10} | {4: <15} | {5: <10} | {6: <10} | {7: <10} | {8: <10} | {9: <10}",
                        sub_group.population.to_formatted_string(&Locale::en),
                        group.state.name,
                        group.profession.name,
                        sub_group.info.race.name,
                        format!("{:?}", sub_group.info.nationality),
                        sub_group.info.ethnicity.name,
                        sub_group.info.religion.name,
                        format!("{:.2} ({:.2})", sub_group.info.wealth.level, sub_group.info.wealth.amount),
                        sub_group.info.income.earned,
                        sub_group.info.sol,
                    )?;
                }
            }
        }

        write!(f, "")
    }
}
