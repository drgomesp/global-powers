use crate::population::group::{Group, SubGroup};
use crate::population::income::Income;
use crate::population::income::Periodicity::Daily;
use crate::population::wealth::Wealth;
use crate::population::StandardOfLiving::{
    Adequate, Excellent, Good, Impoverished, Lavish, Ostentatious, Poor, Prosperous, Struggling,
    Wealthy,
};
use crate::population::{Class, Ethnicity, Profession, Religion};
use crate::region::country::rates::Rates;
use crate::region::country::Country;
use crate::region::state::State;
use crate::region::Region;
use crate::region::Region::{South, Southeast};
use chrono::{DateTime, Datelike, Duration, Local};
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration as StdDuration;

mod population;
mod region;
mod trade;

fn main() {
    let rates = Rates::new(1.3);
    let mut brazil = Country::new("Brazil".into(), rates);

    let states = [
        State::new("SC".into(), "Santa Catarina".into(), Region::South, 2.3),
        State::new("RS".into(), "Rio Grande do Sul".into(), Region::South, 5.8),
        State::new("RJ".into(), "Rio de Janeiro".into(), Region::Southeast, 7.4),
        State::new("SP".into(), "Sao Paulo".into(), Region::Southeast, 21.6),
    ];

    let ethnicities = [
        Ethnicity::new("Mixed".into(), 45.53),
        Ethnicity::new("White".into(), 43.46),
        Ethnicity::new("Black".into(), 10.17),
        // Ethnicity::new("Indigenous".into(), 0.60),
        Ethnicity::new("Asian".into(), 0.42),
    ];

    let religions = [
        Religion::new("Catholic".into(), 64.6),
        Religion::new("Protestant".into(), 24.0),
        Religion::new("African".into(), 3.0),
        // Religion::new("Agnostic".into(), 8.0),
    ];

    let professions = [
        Profession::new(Class::Lower, "Construction Worker".into()),
        Profession::new(Class::LowerMiddle, "Drivers".into()),
        Profession::new(Class::Middle, "Public Worker".into()),
        Profession::new(Class::UpperMiddle, "Influencer".into()),
        Profession::new(Class::Upper, "Business Owner".into()),
    ];

    for mut state in states {
        for profession in &professions {
            let rate = match profession.class {
                Class::Lower => 1,
                Class::LowerMiddle => 10,
                Class::Middle => 20,
                Class::UpperMiddle => 30,
                Class::Upper => 40,
            };

            let mut profession_group = Group::new(profession);

            for ethnicity in &ethnicities {
                for religion in &religions {
                    let population = rand::thread_rng().gen_range(
                        (10.0..(ethnicity.population_percentage * 100.0)
                            * state.population_percentage
                            * religion.population_percentage),
                    );

                    let wealth_level = rand::thread_rng().gen_range(rate..rate + 10);

                    profession_group.add_sub_group(SubGroup::new(
                        ethnicity,
                        religion,
                        Wealth::new(
                            wealth_level,
                            rand::thread_rng().gen_range(1..1_000 * wealth_level),
                        ),
                        Income::new(Daily, 0, 400),
                        match wealth_level {
                            0..5 => Impoverished,
                            5..10 => Struggling,
                            10..15 => Poor,
                            15..20 => Adequate,
                            20..25 => Good,
                            25..30 => Excellent,
                            30..35 => Prosperous,
                            35..40 => Wealthy,
                            40..45 => Lavish,
                            45..50 => Ostentatious,
                            i => panic!("invalid wealth level {i}"),
                        },
                        population as u64,
                    ));
                }
            }

            state.add_group(profession_group)
        }

        brazil.add_state(state);
    }

    let mut day: DateTime<Local> = Local::now();
    let mut year = day.year().clone();

    loop {
        day = day.add(Duration::days(1));

        println!(
            "{0: <10} | {1: <20}\n",
            day.format("%d/%m/%Y"),
            brazil.get_population().to_formatted_string(&Locale::en)
        );

        println!("\n{0: <10} | {1: <10}", "Region", "Population",);

        println!(
            "{0: <10} | {1: <20}",
            "Southeast",
            brazil
                .get_population_by_region(Southeast)
                .to_formatted_string(&Locale::en),
        );

        println!(
            "{0: <10} | {1: <20}",
            "South",
            brazil
                .get_population_by_region(South)
                .to_formatted_string(&Locale::en),
        );

        if day.year() > year {
            year = day.year().clone();

            brazil.update_population();
        }

        print!("\n{:?}", brazil);
        sleep(StdDuration::from_millis(25));

        print!("\x1B[2J\x1B[1;1H");
    }
}
