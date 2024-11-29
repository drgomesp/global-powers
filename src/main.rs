use crate::population::ethnicity::Ethnicity;
use crate::population::group::{Group, SubGroup, SubGroupInfo};
use crate::population::{Class, Profession, Religion};
use crate::region::country::rates::Rates;
use crate::region::country::Country;
use crate::region::state::State;
use crate::region::Region;
use chrono::{DateTime, Datelike, Duration, Local};
use rand::Rng;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration as StdDuration;

mod population;
mod region;
mod trade;

use crate::population::income::Income;
use crate::population::income::Periodicity::Weekly;
use crate::population::nationality::Nationality::Brazilian;
use crate::population::race::Race;
use crate::population::wealth::Wealth;
use crate::population::StandardOfLiving::{
    Adequate, Excellent, Good, Impoverished, Lavish, Ostentatious, Poor, Prosperous, Struggling,
    Wealthy,
};
use crate::region::Region::{South, Southeast};
use num_format::{Locale, ToFormattedString};

fn main() -> eframe::Result {
    let rates = Rates::new(1.3);

    let races = [
        Race::new("White".into()),
        Race::new("Black".into()),
        Race::new("Mixed".into()),
        Race::new("Indigenous".into()),
        Race::new("Asian".into()),
    ];

    let ethnicities = vec![
        Ethnicity::new("Portuguese".into(), 45.53),
        Ethnicity::new("Italian".into(), 13.08),
        Ethnicity::new("Spanish".into(), 14.70),
        Ethnicity::new("German".into(), 18.276),
        Ethnicity::new("Angolan".into(), 2.39),
        Ethnicity::new("Japanese".into(), 0.47),
        // Ethnicity::new("Indigenous".into(), 0.60),
        // Ethnicity::new("Asian".into(), 0.42),
    ];

    let mut brazil = Country::new("Brazil".into(), rates);

    let states = vec![
        State::new("SC".into(), "Santa Catarina".into(), Region::South, 2.3),
        State::new("RS".into(), "Rio Grande do Sul".into(), Region::South, 5.8),
        State::new("RJ".into(), "Rio de Janeiro".into(), Region::Southeast, 7.4),
        State::new("SP".into(), "Sao Paulo".into(), Region::Southeast, 21.6),
    ];

    let religions = [
        Religion::new("Catholic".into(), 64.6),
        Religion::new("Protestant".into(), 24.0),
        Religion::new("African".into(), 3.0),
        // Religion::new("Agnostic".into(), 8.0),
    ];

    let professions = [
        Profession::new(Class::Lower, "Construction Worker".into()),
        // Profession::new(Class::LowerMiddle, "Drivers".into()),
        Profession::new(Class::Middle, "Public Worker".into()),
        // Profession::new(Class::UpperMiddle, "Influencer".into()),
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

            let mut profession_group = Group::new(state.clone(), profession.clone());

            for ethnicity in &ethnicities {
                for religion in &religions {
                    let population = rand::thread_rng().gen_range(
                        10.0..(ethnicity.population_percentage
                            * 10.0
                            * state.population_percentage
                            * religion.population_percentage),
                    );

                    let wealth_level = rand::thread_rng().gen_range(rate..rate + 10);
                    let random_key = rand::thread_rng().gen_range(0..2);
                    let race = &races[random_key];

                    profession_group.add_sub_group(SubGroup::new(
                        SubGroupInfo::new(
                            Brazilian,
                            race,
                            ethnicity,
                            religion,
                            Wealth::new(
                                wealth_level,
                                rand::thread_rng().gen_range(1..1_000 * wealth_level),
                            ),
                            Income::new(
                                Weekly,
                                rand::thread_rng().gen_range(1..1_000 * wealth_level),
                                0,
                            ),
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
                        ),
                        population as u64,
                    ));
                }
            }

            state.add_group(profession_group);
        }

        brazil.add_state(state);
    }

    let mut day: DateTime<Local> = Local::now();
    let mut year = day.year().clone();

    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    //
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };
    //
    // // Our application state:
    // let mut name = "Arthur".to_owned();
    // let mut age = 42;
    //
    // eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         ui.heading("My egui Application");
    //         ui.horizontal(|ui| {
    //             let name_label = ui.label("Your name: ");
    //             ui.text_edit_singleline(&mut name)
    //                 .labelled_by(name_label.id);
    //         });
    //         ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    //         if ui.button("Increment").clicked() {
    //             age += 1;
    //         }
    //         ui.label(format!("Hello '{name}', age {age}"));
    //     });
    // })

    loop {
        day = day.add(Duration::days(1));

        println!(
            "{0: <10} | {1: <10} | {2: <10}",
            "Date", "Country", "Population",
        );

        println!(
            "{0: <10} | {1: <10} | {2: <20}\n",
            day.format("%d/%m/%Y"),
            brazil.name,
            brazil.get_population().to_formatted_string(&Locale::en)
        );

        println!("{0: <10} | {1: <10}", "State", "Population",);

        println!(
            "{0: <10} | {1: <20}",
            "Sao Paulo",
            brazil
                .get_population_by_state("SP".into())
                .to_formatted_string(&Locale::en),
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
        sleep(StdDuration::from_millis(100));

        print!("\x1B[2J\x1B[1;1H");
    }
}
