use cursive::{views::{EditView, Button, LinearLayout}, view::{Nameable, Resizable}, View, direction::Orientation};

pub fn clearable_edit(name: &'static str, default: &str) -> impl View {
    LinearLayout::new(Orientation::Horizontal)
        .child(EditView::new().content(default).with_name(name).fixed_width(47))
        .child(Button::new("X", |s| {
            s.find_name::<EditView>(name).unwrap().set_content("");
        }))
}