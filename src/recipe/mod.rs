#![allow(dead_code)]

use self::unit::Measure;
mod unit;

#[derive(Debug, Clone)]
pub struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ingredient {
    name: String,
    measure: Measure,
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