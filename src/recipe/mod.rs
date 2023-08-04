#![allow(dead_code)]

use serde::{Serialize, Deserialize};

use self::unit::Measure;
pub mod unit;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub(crate) name: String,
    pub(crate) ingredients: Vec<Ingredient>,
    pub(crate) tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub(crate) name: String,
    pub(crate) measure: Measure,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Tag {
    Culture(String),
    Meat(String),
    MealType(MealType),
    PrepType(PrepType),
    Other(String),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Side,
    Snack,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PrepType {
    Cold,
    Bake,
    Fry,
    Microwave,
    Boil,
    Stovetop,
}