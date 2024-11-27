use crate::population::culture::Culture;
use crate::population::ethnicity::Ethnicity;
use crate::population::group::{Group, SubGroup};
use crate::population::heritage::Heritage;
use crate::population::income::Income;
use crate::population::income::Periodicity::Weekly;
use crate::population::nationality::Nationality::Brazilian;
use crate::population::wealth::Wealth;
use crate::population::StandardOfLiving::{
    Adequate, Excellent, Good, Impoverished, Lavish, Ostentatious, Poor, Prosperous, Struggling,
    Wealthy,
};
use crate::population::{Class, Profession, Religion};
use crate::region::country::rates::Rates;
use crate::region::country::Country;
use crate::region::state::State;
use crate::region::Region;
use chrono::{DateTime, Datelike, Local};
use num_format::ToFormattedString;
use rand::Rng;
use std::ops::Add;
use std::time::Duration as StdDuration;

use ratatui::{style::Stylize, widgets::Widget};
use strum::IntoEnumIterator;

pub mod app;
mod population;
mod region;
mod trade;

fn main() -> Result<()> {
    let rates = Rates::new(1.3);

    let ethnicities = vec![
        Ethnicity::new("Brown".into(), 45.53),
        Ethnicity::new("White".into(), 43.46),
        Ethnicity::new("White".into(), 43.46),
        Ethnicity::new("Black".into(), 10.17),
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
        Profession::new(Class::LowerMiddle, "Drivers".into()),
        Profession::new(Class::Middle, "Public Worker".into()),
        Profession::new(Class::UpperMiddle, "Influencer".into()),
        Profession::new(Class::Upper, "Business Owner".into()),
    ];

    let cultures = vec![
        Culture::new("Paulista".into()),
        Culture::new("Gaucho".into()),
        Culture::new("Nordestino".into()),
    ];

    let heritages = [
        Heritage::new("Portuguese".into()),
        Heritage::new("Italian".into()),
        Heritage::new("German".into()),
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
                        (10.0..(ethnicity.population_percentage * 10.0)
                            * state.population_percentage
                            * religion.population_percentage),
                    );

                    let wealth_level = rand::thread_rng().gen_range(rate..rate + 10);
                    let random_key = rand::thread_rng().gen_range(0..2);
                    let heritage = &heritages[random_key];
                    let culture = &cultures[random_key];

                    profession_group.add_sub_group(SubGroup::new(
                        Brazilian,
                        culture,
                        ethnicity,
                        heritage,
                        religion,
                        Wealth::new(
                            wealth_level,
                            rand::thread_rng().gen_range(1..1_000 * wealth_level),
                        ),
                        Income::new(Weekly, 0, 400),
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

            state.add_group(profession_group);
        }

        brazil.add_state(state);
    }

    let mut day: DateTime<Local> = Local::now();
    let mut year = day.year().clone();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();

    app_result

    // loop {
    //     day = day.add(Duration::days(1));
    //
    //     println!(
    //         "{0: <10} | {1: <10} | {2: <10}",
    //         "Date", "Country", "Population",
    //     );
    //
    //     println!(
    //         "{0: <10} | {1: <10} | {2: <20}\n",
    //         day.format("%d/%m/%Y"),
    //         brazil.name,
    //         brazil.get_population().to_formatted_string(&Locale::en)
    //     );
    //
    //     println!("{0: <10} | {1: <10}", "State", "Population",);
    //
    //     println!(
    //         "{0: <10} | {1: <20}",
    //         "Sao Paulo",
    //         brazil
    //             .get_population_by_state("SP".into())
    //             .to_formatted_string(&Locale::en),
    //     );
    //
    //     println!("\n{0: <10} | {1: <10}", "Region", "Population",);
    //
    //     println!(
    //         "{0: <10} | {1: <20}",
    //         "Southeast",
    //         brazil
    //             .get_population_by_region(Southeast)
    //             .to_formatted_string(&Locale::en),
    //     );
    //
    //     println!(
    //         "{0: <10} | {1: <20}",
    //         "South",
    //         brazil
    //             .get_population_by_region(South)
    //             .to_formatted_string(&Locale::en),
    //     );
    //
    //     if day.year() > year {
    //         year = day.year().clone();
    //
    //         brazil.update_population();
    //     }
    //
    //     print!("\n{:?}", brazil);
    //     sleep(StdDuration::from_millis(100));
    //
    //     print!("\x1B[2J\x1B[1;1H");
    // }
}

use crate::app::App;
use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(StdDuration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
