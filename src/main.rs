use crate::population::{Age, Class, Ethnicity, Group, Profession, Religion, SubGroup};
use crate::region::{Country, State};
use rand::Rng;
use strum::IntoEnumIterator;

mod population;
mod region;

fn main() {
    let mut brazil = Country::new();

    let states = [
        State::new("SC".into(), "Santa Catarina".into()),
        State::new("RS".into(), "Rio Grande do Sul".into()),
        State::new("SP".into(), "Sao Paulo".into()),
    ];

    let ethnicities = [
        Ethnicity::new("Mixed".into()),
        Ethnicity::new("White".into()),
        Ethnicity::new("Black".into()),
        Ethnicity::new("Indigenous".into()),
        Ethnicity::new("Asian".into()),
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
                        let population = rand::thread_rng().gen_range(0..10_000);
                        group.add_sub_group(SubGroup::new(ethnicity, religion, population));
                    }
                }

                state.add_group(group)
            }
        }

        brazil.add_state(state);
    }

    println!("{:#?}", brazil);
}
