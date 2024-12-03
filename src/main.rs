use crate::population::group::{Group, SubGroup, SubGroupInfo};
use crate::population::heritage::Heritage;
use crate::population::{Class, Profession, Religion};
use crate::region::country::rates::Rates;
use crate::region::country::Country;
use crate::region::state::State;
use crate::region::Region;
use chrono::{DateTime, Datelike, Duration, Local};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use rand::Rng;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration as StdDuration;
mod population;
mod region;
mod trade;

use crate::population::ethnicity::Ethnicity;
use crate::population::income::Income;
use crate::population::income::Periodicity::Weekly;
use crate::population::nationality::Nationality::Brazilian;
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
        Ethnicity::new("White".into()),
        Ethnicity::new("Black".into()),
        Ethnicity::new("Brown".into()),
        Ethnicity::new("Indigenous".into()),
        Ethnicity::new("Asian".into()),
    ];

    let ethnicities = vec![
        Heritage::new("Portuguese".into(), 45.53),
        Heritage::new("Italian".into(), 13.08),
        Heritage::new("Spanish".into(), 14.70),
        Heritage::new("German".into(), 18.276),
        Heritage::new("Angolan".into(), 2.39),
        Heritage::new("Japanese".into(), 0.47),
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
                    let random_key = rand::thread_rng().gen_range(0..3);
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

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (Heading, FontId::new(20.0, Proportional)),
                (Body, FontId::new(20.0, Proportional)),
                (Monospace, FontId::new(20.0, Proportional)),
                (Button, FontId::new(20.0, Proportional)),
                (Small, FontId::new(20.0, Proportional)),
            ]
            .into();
            ctx.set_style(style);

            egui::TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Expandable Upper Panel");
                        });
                    });
                });
        });
    })?;

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

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label("ASDASD");
            ui.add(egui::Separator::default().grow(8.0));
            ui.label("ASDASD");
        },
    );
}
