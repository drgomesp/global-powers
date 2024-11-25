pub mod group;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq)]
pub enum Class {
    Upper,
    Middle,
    Lower,
}

#[derive(Debug, EnumIter)]
pub enum Age {
    Children,
    Adult,
    Senior,
}

#[derive(Debug, PartialEq)]
pub struct Profession {
    class: Class,
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

#[derive(Debug)]
pub struct Ethnicity {
    pub name: String,
    pub population_percentage: f64,
}

impl Ethnicity {
    pub fn new(name: String, percentage: f64) -> Self {
        Self {
            name,
            population_percentage: percentage,
        }
    }
}
