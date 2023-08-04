#![allow(dead_code)]

use self::unit::{Measure, Unit};
pub mod unit;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub(crate) name: String,
    pub(crate) ingredients: Vec<Ingredient>,
    pub(crate) tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ingredient {
    pub(crate) name: String,
    pub(crate) measure: Measure,
    pub(crate) primary_unit: Unit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tag {
    Culture(String),
    Meat(String),
    MealType(MealType),
    PrepType(PrepType),
    Other(String),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Side,
    Snack,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PrepType {
    Cold,
    Bake,
    Fry,
    Microwave,
    Boil,
}