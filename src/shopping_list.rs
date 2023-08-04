use std::collections::{HashMap, HashSet};

use crate::recipe::{Recipe, unit::Measure};

type IngredientMap = HashMap<String, Measure>;
type Associations = HashMap<String, HashSet<String>>;

pub fn generate_list(list: Vec<Recipe>) -> ShoppingList {
    let mut ingredients = IngredientMap::new();
    let mut associations = Associations::new();

    list.into_iter()
        .for_each(|recipe| {
            for ingredient in recipe.ingredients {

                //Get the current entry for this ingredient, or insert an empty default using the
                //primary unit of measure for it.
                let current = ingredients
                    .entry(ingredient.name.clone())
                    .or_insert(Measure::new(ingredient.primary_unit));

                //Always promote to the next highest unit, if needed.
                if current.unit >= ingredient.measure.unit {
                    current.quantity += ingredient.measure.convert_to(current.unit).quantity;
                } else {
                    *current = current.convert_to(ingredient.measure.unit);
                    current.quantity += ingredient.measure.quantity;
                }

                //And finally, associate the ingredient with the recipe
                associations.entry(ingredient.name).or_insert(HashSet::new()).insert(recipe.name.clone());
            }
        });

    ShoppingList { ingredients, associations }
}

pub struct ShoppingList {
    ingredients: IngredientMap,
    associations: Associations,
}