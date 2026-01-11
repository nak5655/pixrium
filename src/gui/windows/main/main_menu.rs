use crate::core::logics::commands::ShowVersionCommand;
use crate::core::logics::Console;
use crate::FreyaServices;
use freya::prelude::*;

pub fn main_menu(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    let mut show_menu = use_state(|| false);

    rect()
        .child(
            Button::new()
                .on_press(move |_| show_menu.toggle())
                .child("Menu"),
        )
        .maybe_child(show_menu().then(|| {
            Menu::new()
                .on_close(move |_| show_menu.set(false))
                .child(
                    SubMenu::new()
                        .label("File")
                        .child(MenuButton::new().child("Open"))
                        .child(MenuButton::new().child("Save"))
                        .child(MenuButton::new().child("Close")),
                )
                .child(
                    SubMenu::new().label("Help").child(
                        MenuButton::new()
                            .on_press(move |_| console.write().execute(&ShowVersionCommand {}))
                            .child("Version"),
                    ),
                )
        }))
}
