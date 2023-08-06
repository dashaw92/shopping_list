// MVC approach adapted from:
// https://cafbit.com/post/cursive_writing_terminal_applications_in_rust/

use cursive::{
    align::HAlign,
    direction::{Orientation, Direction},
    event::Key,
    menu::Tree,
    view::{Nameable, Scrollable},
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, TextView},
    Cursive, CursiveRunnable, CursiveRunner, With, View,
};

use crate::{app::AppState, shopping_list};

use std::sync::mpsc;

const HELP: &str = "Select all the recipes you wish to have a shopping list for.
Use the generate button to start the report wizard, which will
guide you through the steps of exporting the generated shopping list.";

const ABOUT: &str = "Shopping List Generator
by dashaw92 - August 2023
";

const SAVE: &str = "Choose where to save the shopping list to.
Reports are in plaintext format (txt).
";

fn build_ui(ui: &mut Ui, list: Vec<String>) {
    let ctrl_tx = ui.controller.clone();

    ui.siv.with_theme(|theme| {
        use cursive::theme::BaseColor;
        use cursive::theme::PaletteColor;

        theme.palette[PaletteColor::Background] = BaseColor::Green.dark();
        theme.palette[PaletteColor::Shadow] = BaseColor::Black.dark();
    });

    ui.siv
        .menubar()
        .add_subtree("File", Tree::new().leaf("Quit", Cursive::quit))
        .add_subtree(
            "About",
            Tree::new()
                .leaf("Help", |s| s.add_layer(Dialog::info(HELP).title("Help")))
                .delimiter()
                .leaf("About", |s| s.add_layer(Dialog::info(ABOUT).title("About"))),
        );
    ui.siv.set_autohide_menu(false);
    ui.siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    let recipe_list = Dialog::around(
        ListView::new()
            .with(|view| {
                for item in list {
                    let ctrl = ctrl_tx.clone();
                    let item_closure = item.clone();
                    let check = Checkbox::new()
                        .with_checked(false)
                        .on_change(move |_, state| {
                            let _ = ctrl.send(ControllerMessage::UpdateSelected(
                                item_closure.clone(),
                                state,
                            ));
                        });
                    view.add_child(&item, check);
                }
            })
            .with_name("list")
            .scrollable(),
    )
    .title("Recipes")
    .title_position(HAlign::Left)
    .button("Generate", move |s| {
        render_export(s, ctrl_tx.clone());
    });
    ui.siv.add_layer(recipe_list);
}

fn render_export(siv: &mut Cursive, ctrl: mpsc::Sender<ControllerMessage>) {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().map(ToOwned::to_owned).unwrap_or("".into());

    siv.add_layer(
        Dialog::around(
            LinearLayout::new(Orientation::Vertical)
                .child(TextView::new(SAVE))
                .child(ListView::new()
                    .delimiter()
                    .child("File name", EditView::new().filler(" ").with_name("filename"))
                    .child("CWD", TextView::new(path))
                ),
        )
        .title("Save as...")
        .dismiss_button("Cancel")
        .button("Save", move |s| {
            let name = s.find_name::<EditView>("filename").unwrap().get_content();
            if name.trim().is_empty() {
                return;
            }

            s.pop_layer();

            let _ = ctrl.send(ControllerMessage::ExportList(name.to_string()));
        }),
    );

    if let Some(mut view) = siv.find_name::<EditView>("filename") {
        let _ = view.take_focus(Direction::none());
        return
    };

}

enum ControllerMessage {
    UpdateSelected(String, bool),
    ExportList(String),
}

struct Ui {
    siv: CursiveRunner<CursiveRunnable>,
    controller: mpsc::Sender<ControllerMessage>,
}

impl Ui {
    fn new(tx: mpsc::Sender<ControllerMessage>, list: Vec<String>) -> Ui {
        let mut ui = Ui {
            siv: cursive::default().into_runner(),
            controller: tx,
        };

        build_ui(&mut ui, list);
        ui
    }

    fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        }

        self.siv.step();
        true
    }
}

pub struct Controller {
    ui: Ui,
    rx: mpsc::Receiver<ControllerMessage>,
    state: AppState,
}

impl Controller {
    pub fn new(state: AppState) -> Controller {
        let (tx, rx) = mpsc::channel();

        let list = state
            .recipes()
            .iter()
            .map(|recipe| recipe.name.clone())
            .collect();

        Controller {
            ui: Ui::new(tx.clone(), list),
            rx,
            state,
        }
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            while let Some(msg) = self.rx.try_iter().next() {
                match msg {
                    ControllerMessage::UpdateSelected(recipe, selected) => {
                        if selected {
                            self.state.select(recipe);
                        } else {
                            self.state.unselect(recipe);
                        }
                    }
                    ControllerMessage::ExportList(path) => {
                        let list = shopping_list::generate_list(self.state.selected());
                        list.render_to(&path);
                    }
                }
            }
        }
    }
}
