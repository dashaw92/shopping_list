use cursive::{
    event::Key,
    menu::Tree,
    view::Scrollable,
    views::{Dialog, SelectView},
    With,
};

use crate::app::AppState;

const HELP: &str = "Select all the recipes you wish to have a shopping list for.
Use the generate button to start the report wizard, which will
guide you through the steps of exporting the generated shopping list.";

const ABOUT: &str = "Shopping List Generator
by dashaw92 - August 2023
";

pub fn run(app: AppState) {
    let mut siv = cursive::default();
    siv.set_user_data(app);
    siv.menubar()
        .add_subtree("File", Tree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "Recipes",
            Tree::new().leaf("Show Recipes",  |s| {
                let app: AppState = s.with_user_data(|app: &mut AppState| app.clone()).unwrap();
                s.add_layer(
                    Dialog::around(
                        SelectView::new()
                            .with(|list| {
                                app.recipes()
                                    .iter()
                                    .map(|recipe| &recipe.name)
                                    .for_each(|name| list.add_item_str(name));
                            })
                            .scrollable(),
                    )
                    .dismiss_button("Close"),
                )
            }),
        )
        .add_subtree(
            "About",
            Tree::new()
                .leaf("Help", |s| s.add_layer(Dialog::info(HELP)))
                .delimiter()
                .leaf("About", |s| s.add_layer(Dialog::info(ABOUT).title("About"))),
        );
    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Tab, |s| s.select_menubar());
    siv.run();
}
