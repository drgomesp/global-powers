use crate::population::{Age, Class, Ethnicity, Group, Profession, Religion, SubGroup};
use crate::region::{Country, State};
use rand::Rng;
use strum::IntoEnumIterator;

mod population;
mod region;

fn main() {
    let mut brazil = Country::new();

    let states = [
        State::new("SC".into(), "Santa Catarina".into(), 2.3),
        State::new("RS".into(), "Rio Grande do Sul".into(), 7.4),
        State::new("SP".into(), "Sao Paulo".into(), 21.6),
    ];

    let ethnicities = [
        Ethnicity::new("Mixed".into(), 45.53),
        Ethnicity::new("White".into(), 43.46),
        Ethnicity::new("Black".into(), 10.17),
        Ethnicity::new("Indigenous".into(), 0.60),
        Ethnicity::new("Asian".into(), 0.42),
    ];

    let religions = [
        Religion::new("Catholic".into()),
        Religion::new("Protestant".into()),
        Religion::new("African".into()),
        Religion::new("Agnostic".into()),
        Religion::new("Other".into()),
    ];

    let professions = [
        Profession::new(Class::Lower, "Construction Worker".into()),
        Profession::new(Class::Middle, "Public Worker".into()),
        Profession::new(Class::Upper, "Influencer".into()),
    ];

    let unemployed = Profession::new(Class::Lower, "Unemployed".into());

    for mut state in states {
        for age in Age::iter() {
            for profession in &professions {
                let mut group = match age {
                    Age::Children | Age::Senior => Group::new(state.id.clone(), &unemployed),
                    Age::Adult => Group::new(state.id.clone(), profession),
                };

                for ethnicity in &ethnicities {
                    for religion in &religions {
                        let population = rand::thread_rng().gen_range(
                            (100.0..(ethnicity.percentage * 1000.0) * state.population_percentage),
                        );
                        group.add_sub_group(SubGroup::new(ethnicity, religion, population as u64));
                    }
                }

                state.add_group(group)
            }
        }

        brazil.add_state(state);
    }

    println!("SC => {:#?}", brazil.get_state_population("SC").unwrap());
    println!("RS => {:#?}", brazil.get_state_population("RS").unwrap());
    println!("SP => {:#?}", brazil.get_state_population("SP").unwrap());

    println!("BR => {:#?}", brazil.population);
}
