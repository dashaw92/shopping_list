#![allow(dead_code)]


use recipe::Recipe;

use crate::{recipe::{Ingredient, unit::{Measure, Unit}, Tag, MealType, PrepType}, app::AppState};

mod app;
mod recipe;
mod shopping_list;

fn main() {
    let mut app = AppState::load_from_dir("recipes");
    let tacos = Recipe {
        name: "Sante Fe Pork Tacos".into(),
        ingredients: vec![
            Ingredient { name: "Yellow Onion".into(), measure: Measure { quantity: 1.0, unit: Unit::Whole } },
            Ingredient { name: "Cilantro".into(), measure: Measure { quantity: 0.25, unit: Unit::Ounces } },
            Ingredient { name: "Lime".into(), measure: Measure { quantity: 1.0, unit: Unit::Whole } },
            Ingredient { name: "Ground Pork".into(), measure: Measure { quantity: 10.0, unit: Unit::Ounces } },
            Ingredient { name: "Southwest Spice Blend".into(), measure: Measure { quantity: 1.0, unit: Unit::Tablespoons } },
            Ingredient { name: "Red Cabbage".into(), measure: Measure { quantity: 4.0, unit: Unit::Ounces } },
            Ingredient { name: "Mayonnaise".into(), measure: Measure { quantity: 2.0, unit: Unit::Tablespoons } },
            Ingredient { name: "Tex Mex Paste".into(), measure: Measure { quantity: 1.0, unit: Unit::Whole } },
            Ingredient { name: "Tortillas".into(), measure: Measure { quantity: 6.0, unit: Unit::Whole } },
            Ingredient { name: "Monterey Jack, Shredded".into(), measure: Measure { quantity: 0.25, unit: Unit::Cups } },
            Ingredient { name: "Sour Cream".into(), measure: Measure { quantity: 1.5, unit: Unit::Tablespoons } },
        ],
        tags: Some(vec![
            Tag::Meat("Pork".into()),
            Tag::Culture("Mexican".into()),
            Tag::MealType(MealType::Lunch),
            Tag::MealType(MealType::Dinner),
            Tag::PrepType(PrepType::Stovetop)
        ]),
    };

    if app.recipe_by_name(&tacos.name).is_none() {
        app.add_recipe(tacos);
    }

    println!("Loaded {} recipes from disk!", app.recipes().len());
}

