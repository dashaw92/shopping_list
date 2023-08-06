use std::{collections::{HashMap, HashSet}, fs::OpenOptions, io::Write, path::PathBuf};

use crate::recipe::{Recipe, unit::Measure};

type IngredientMap = HashMap<String, Measure>;
type Associations = HashMap<String, HashSet<String>>;

pub fn generate_list(list: &[Recipe]) -> ShoppingList {
    let mut ingredients = IngredientMap::new();
    let mut associations = Associations::new();

    list.iter()
        .for_each(|recipe| {
            for ingredient in &recipe.ingredients {
                //Get the current entry for this ingredient, or insert an empty default using the
                //primary unit of measure for it.
                let current = ingredients
                    .entry(ingredient.name.clone())
                    .or_insert(Measure::new(ingredient.measure.unit));

                //Always promote to the next highest unit, if needed.
                if current.unit >= ingredient.measure.unit {
                    current.quantity += ingredient.measure.convert_to(current.unit).quantity;
                } else {
                    *current = current.convert_to(ingredient.measure.unit);
                    current.quantity += ingredient.measure.quantity;
                }

                //And finally, associate the ingredient with the recipe
                associations.entry(ingredient.name.clone()).or_insert(HashSet::new()).insert(recipe.name.clone());
            }
        });

    ShoppingList { ingredients, associations }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ShoppingList {
    ingredients: IngredientMap,
    associations: Associations,
}

pub enum ExportFormat {
    Print,
    Notes,
}

impl ShoppingList {
    pub fn render_to(&self, path: &str, name: &str, format: ExportFormat) {
        if self.ingredients.is_empty() {
            return
        }

        let recipes: HashSet<String> = self.associations.values()
            .flat_map(|set| set.iter())
            .map(Clone::clone)
            .collect();

        let recipes: Vec<String> = recipes.into_iter().collect();

        let mut report = match format {
            ExportFormat::Print => self.print_report(&recipes),
            ExportFormat::Notes => self.notes_report(&recipes),
        };
        report.write_to(path, name);
    }

    fn print_report(&self, recipes: &[String]) -> Report {
        let mut report = Report::default();
        report.line();
        report.w("My Shopping List");
        report.line();
        for (ingredient, measure) in &self.ingredients {
            report.w(format!("[_] {ingredient}"));
            report.w(format!("    * {} {:?}", measure.quantity.ceil(), measure.unit));
        }
        report.line();
        report.w("Recipes:");
        for recipe in recipes {
            report.w(format!(" - {recipe}"));
        }
        report.line();
        report
    }

    fn notes_report(&self, recipes: &[String]) -> Report {
        let mut report = Report {
            border: false,
            ..Default::default()
        };

        report.w("My Shopping List");
        report.line();
        for (ingredient, measure) in &self.ingredients {
            report.w(format!("{ingredient}: {} {:?}", measure.quantity.ceil(), measure.unit));
        }

        report.w("");
        report.w("Recipes:");
        for recipe in recipes {
            report.w(format!(" - {recipe}"));
        }
        
        report
    }
}

struct Report {
    buf: Vec<String>,
    lines: Vec<usize>,
    lborder: char,
    rborder: char,
    tborder: char,
    cborder: char,
    border: bool,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            buf: Vec::new(),
            lines: Vec::new(),
            lborder: '|',
            rborder: '|',
            tborder: '-',
            cborder: '+',
            border: true,
        }
    }
}

impl Report {
    fn line(&mut self) {
        self.w("");
        self.lines.push(self.buf.len() - 1);
    }

    fn w<S: ToString>(&mut self, line: S) {
        self.buf.push(format!("{}{}", if self.border {
            format!("{} ", self.lborder)
        } else {
            "".into()
        }, line.to_string()));
    }
    
    fn render(&mut self) {
        let max_len = self.buf.iter().map(String::len).max().unwrap_or(52) + 1;
        for idx in 0..self.buf.len() {
            if self.lines.contains(&idx) {
                self.buf[idx] = format!("{}{}{}", self.cborder, self.tborder.to_string().repeat(max_len - 1), self.cborder);
                continue
            }

            if self.border {
                let line = &mut self.buf[idx];
                let padding = max_len - line.len();
                line.push_str(&format!("{}{}", " ".repeat(padding), self.rborder));
            }
        }
    }

    fn write_to(&mut self, path: &str, file: &str) {
        let mut path = if path.is_empty() {
            std::env::current_dir().unwrap()
        } else {
            PathBuf::from(path)
        };
        path.push(file);

        let Ok(mut output) = OpenOptions::new().write(true).create(true).truncate(true).open(path) else {
            return
        };

        self.render();
        let lines = self.buf.join("\n");
        let _ = output.write_all(lines.as_bytes());
    }
}