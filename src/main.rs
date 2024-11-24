use crate::population::group::{Group, SubGroup};
use crate::population::{Class, Ethnicity, Profession, Religion};
use crate::region::country::Country;
use crate::region::state::State;
use crate::region::Region;
use rand::Rng;

mod population;
mod region;

fn main() {
    let mut brazil = Country::new("Brazil".into());

    let states = [
        State::new("SC".into(), "Santa Catarina".into(), Region::South, 2.3),
        State::new("RS".into(), "Rio Grande do Sul".into(), Region::South, 5.8),
        State::new(
            "RJ".into(),
            "Rio Grande do Sul".into(),
            Region::Southeast,
            7.4,
        ),
        State::new("SP".into(), "Sao Paulo".into(), Region::Southeast, 21.6),
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

    for mut state in states {
        for profession in &professions {
            let mut profession_group = Group::new(profession);

            for ethnicity in &ethnicities {
                for religion in &religions {
                    let population = rand::thread_rng().gen_range(
                        (100.0
                            ..(ethnicity.population_percentage * 1000.0)
                                * state.population_percentage),
                    );

                    profession_group.add_sub_group(SubGroup::new(
                        ethnicity,
                        religion,
                        population as u64,
                    ));
                }
            }

            state.add_group(profession_group)
        }

        brazil.add_state(state);
    }

    println!("{:?}", brazil);
}
