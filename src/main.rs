use crate::app::AppState;

mod app;
mod recipe;
mod shopping_list;
mod ui;

fn main() {
    let app = AppState::load_from_dir("recipes");
    println!("Loaded {} recipes from disk!", app.recipes().len());
    let mut ui = ui::Controller::new(app);
    ui.run();
}