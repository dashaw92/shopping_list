#![allow(dead_code)]

use std::path::Path;

use recipe::Recipe;

use crate::recipe::{Ingredient, unit::{Measure, Unit}, Tag, MealType, PrepType};

mod recipe;
mod shopping_list;

fn main() {
    let mut app = App::load("recipes.json");
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

    println!("Loaded {} recipe(s) from disk!", app.recipes.len());
    if app.recipe_by_name(&tacos.name).is_none() {
        app.add_recipe(tacos);
        app.save("recipes.json");
    }
}

struct App {
    recipes: Vec<Recipe>,
    selected: Vec<Recipe>,
}

impl App {
    fn load<P: AsRef<Path>>(path: P) -> Self {
        use std::fs::{File, read_to_string};

        if !path.as_ref().exists() {
            File::create(&path).expect("Failed to create the file!");
        }

        let recipes: Vec<Recipe> = match read_to_string(path)
            .map(|st| serde_json::from_str(&st))
            .expect("Failed to load the recipe DB.") {
                Ok(recipes) => recipes, 
                _ => Vec::new(),
            };

        Self {
            selected: Vec::with_capacity(recipes.len()),
            recipes,
        }
    }

    fn add_recipe(&mut self, recipe: Recipe) {
        if self.recipe_by_name(&recipe.name).is_some() {
            return
        }

        self.recipes.push(recipe);
    }

    fn save<P: AsRef<Path>>(&self, path: P) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new().write(true).open(path).expect("Failed to open recipe DB for saving.");
        serde_json::to_string_pretty(&self.recipes)
            .map(|json| file.write_all(json.as_bytes()))
            .expect("Failed to serialize recipes.")
            .expect("Failed to save recipes to disk.");
    }

    fn recipe_by_name(&self, name: &str) -> Option<Recipe> {
        self.recipes.iter()
            .find(|recipe| recipe.name == name)
            .cloned()
    }

    fn select(&mut self, recipe: Recipe) {
        if self.selected.contains(&recipe) {
            return
        }

        self.selected.push(recipe);
    }

    fn unselect(&mut self, recipe: Recipe) {
        let Some(idx) = self.selected.iter().position(|other| other.name == recipe.name) else {
            return;
        };

        self.selected.remove(idx);
    }
}