pub mod ethnicity;
pub mod group;
pub mod income;
pub mod nationality;
pub mod need;
pub mod race;
pub mod wealth;

use std::fmt::Formatter;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Age {
    Children,
    Adult,
    Senior,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Class {
    Upper,
    UpperMiddle,
    Middle,
    LowerMiddle,
    Lower,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StandardOfLiving {
    Impoverished,
    Struggling,
    Poor,
    Adequate,
    Good,
    Excellent,
    Prosperous,
    Wealthy,
    Lavish,
    Ostentatious,
}

impl std::fmt::Display for StandardOfLiving {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Profession {
    pub class: Class,
    pub name: String,
}

impl Profession {
    pub fn new(class: Class, name: String) -> Self {
        Self { class, name }
    }
}

#[derive(Debug)]
pub struct Religion {
    pub name: String,
    pub population_percentage: f64,
}

impl Religion {
    pub fn new(name: String, population_percentage: f64) -> Self {
        Self {
            name,
            population_percentage,
        }
    }
}
