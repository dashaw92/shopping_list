// MVC approach adapted from:
// https://cafbit.com/post/cursive_writing_terminal_applications_in_rust/

use cursive::{
    event::Key,
    menu::Tree,
    view::{Scrollable, Nameable},
    views::{Dialog, ListView, Checkbox},
    CursiveRunner, CursiveRunnable, align::HAlign, With,
};

use crate::app::AppState;

use std::sync::mpsc;

const HELP: &str = "Select all the recipes you wish to have a shopping list for.
Use the generate button to start the report wizard, which will
guide you through the steps of exporting the generated shopping list.";

const ABOUT: &str = "Shopping List Generator
by dashaw92 - August 2023
";

fn build_ui(ui: &mut Ui, list: Vec<String>) {
    ui.siv.menubar()
        .add_subtree("File", Tree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "About",
            Tree::new()
                .leaf("Help", |s| s.add_layer(Dialog::info(HELP)))
                .delimiter()
                .leaf("About", |s| s.add_layer(Dialog::info(ABOUT).title("About"))),
        );
    ui.siv.set_autohide_menu(false);
    ui.siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    let recipe_list = Dialog::around(
        ListView::new()
            .with(|view| {
                for item in list {
                    let ctrl = ui.controller.clone();
                    let item_closure = item.clone();
                    let check = Checkbox::new().with_checked(false)
                    .on_change(move |_, state| {
                        let _ = ctrl.send(ControllerMessage::UpdateSelected(item_closure.clone(), state));
                    });
                    view.add_child(&item, check);
                }
            })
            .with_name("list")
            .scrollable()
    )
        .title("Recipes")
        .title_position(HAlign::Left)
        .button("Generate", |_b| {});
    ui.siv.add_layer(recipe_list);
}

enum UiMessage {}

enum ControllerMessage {
    UpdateSelected(String, bool),
}

struct Ui {
    siv: CursiveRunner<CursiveRunnable>,
    ui_rx: mpsc::Receiver<UiMessage>,
    ui_tx: mpsc::Sender<UiMessage>,
    controller: mpsc::Sender<ControllerMessage>,
}

impl Ui {
    fn new(tx: mpsc::Sender<ControllerMessage>, list: Vec<String>) -> Ui {
        let (ui_tx, ui_rx) = mpsc::channel();

        let mut ui = Ui {
            siv: cursive::default().into_runner(),
            ui_tx,
            ui_rx,
            controller: tx,
        };

        build_ui(&mut ui, list);
        ui
    }

    fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false
        }

        while let Some(msg) = self.ui_rx.try_iter().next() {
            match msg {}
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
        
        let list = state.recipes().iter()
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
                }
            }
        }
    }
}