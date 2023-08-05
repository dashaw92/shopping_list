use std::{path::{Path, PathBuf}, error::Error};

use crate::recipe::Recipe;

#[derive(Clone)]
pub struct AppState {
    base_dir: PathBuf,
    recipes: Vec<Recipe>,
    selected: Vec<Recipe>,
}

impl AppState {
    pub fn load_from_dir<P: AsRef<Path>>(path: P) -> Self {
        if !path.as_ref().is_dir() {
            if path.as_ref().exists() {
                eprintln!("Error: Requested recipe directory exists and is a file.");
                eprintln!("Please use a different directory, or rename the existing file.");
                std::process::exit(-1)
            }

            if let Err(e) = std::fs::create_dir(path.as_ref()) {
                eprintln!("Could not create recipe directory: {e:?}");
                std::process::exit(-1)
            }

            println!("Created recipe directory.");
        }

        let Ok(files) = std::fs::read_dir(path.as_ref()) else {
            eprintln!("Could not read recipe directory.");
            std::process::exit(-2)
        };

        let mut app = AppState {
            base_dir: path.as_ref().to_path_buf(),
            recipes: vec![],
            selected: vec![],
        };

        files.into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|file| file.is_file())
            .filter_map(|recipe| app.load(recipe.as_path()).err())
            .for_each(|(name, err)| eprintln!("Error loading recipe \"{name}\": {err}"));

        app
    }

    fn load<P: AsRef<Path> + Copy>(&mut self, path: P) -> Result<(), (String, Box<dyn Error>)> {
        use std::fs::read_to_string;

        let recipe: Recipe = match read_to_string(path)
            .map(|st| serde_json::from_str(&st))
            .expect("Failed to load a recipe file.") 
        {
                Ok(recipe) => recipe, 
                Err(e) => {
                    let name = path.as_ref().file_name()
                        .and_then(|ostr| ostr.to_str())
                        .map(ToOwned::to_owned)
                        .unwrap_or(format!("{}", path.as_ref().display()));
                    return Err((name, Box::new(e)))
                },
        };

        println!("Loaded recipe \"{}\".", &recipe.name);
        self.add_recipe(recipe);
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        if self.recipe_by_name(&recipe.name).is_some() {
            return
        }

        self.recipes.push(recipe);
    }

    fn save(&self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        self.recipes.iter()
            .for_each(|recipe| {
                let mut base = self.base_dir.clone();
                base.push(format!("{}.json", recipe.name));

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(base.as_path())
                    .expect("Failed to open recipe file for saving.");

                serde_json::to_string_pretty(recipe)
                    .map(|json| file.write_all(json.as_bytes()))
                    .expect("Failed to serialize recipe.")
                    .expect("Failed to save recipe to disk.");
            });
    }

    pub fn recipe_by_name(&self, name: &str) -> Option<Recipe> {
        self.recipes.iter()
            .find(|recipe| recipe.name == name)
            .cloned()
    }

    pub fn select(&mut self, recipe: String) {
        let Some(recipe) = self.recipe_by_name(&recipe) else {
            return
        };

        if self.selected.contains(&recipe) {
            return
        }

        self.selected.push(recipe);
    }

    pub fn unselect(&mut self, recipe: String) {
        let Some(recipe) = self.recipe_by_name(&recipe) else {
            return
        };

        let Some(idx) = self.selected.iter().position(|other| other.name == recipe.name) else {
            return;
        };

        self.selected.remove(idx);
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn is_selected(&self, name: &str) -> bool {
        self.selected.iter().find(|recipe| recipe.name == name).is_some()
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.save();
    }
}