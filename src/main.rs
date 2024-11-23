use crate::population::{Class, Ethnicity, Group, Profession, Religion, SubGroup};
use crate::region::{Country, State};
use rand::Rng;

mod population;
mod region;

fn main() {
    let mut brazil = Country::new();

    let mut sc = State::new("SC".into(), "Santa Catarina".into());
    let mut rs = State::new("RS".into(), "Rio Grande do Sul".into());
    let mut sp = State::new("SP".into(), "Sao Paulo".into());

    let white = Ethnicity::new("White".into());
    let mixed = Ethnicity::new("Mixed".into());
    let black = Ethnicity::new("Black".into());

    let catholic = Religion::new("Catholic".into());
    let protestant = Religion::new("Protestant".into());
    let agnostic = Religion::new("Agnostic".into());

    let construction_worker = Profession::new(Class::Lower, "Construction Worker".into());
    let public_worker = Profession::new(Class::Middle, "Public Worker".into());
    let influencer = Profession::new(Class::Upper, "Influencer".into());

    for mut state in [sc, rs, sp] {
        for profession in [&construction_worker, &public_worker, &influencer] {
            let mut group = Group::new(state.id.clone(), profession);

            for ethnicity in [&white, &mixed, &black] {
                for religion in [&catholic, &protestant, &agnostic] {
                    let population = rand::thread_rng().gen_range(0..10_000);

                    group.add_sub_group(SubGroup::new(ethnicity, religion, population));
                }
            }

            state.add_group(group)
        }

        brazil.add_state(state);
    }

    println!("{:#?}", brazil);
}
