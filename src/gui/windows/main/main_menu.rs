use crate::core::input::commands::{OpenFileCommand, ShowVersionCommand};
use crate::core::input::InputSignal;
use crate::FreyaServices;
use freya::prelude::*;
use std::sync::mpsc::Sender;

pub fn main_menu(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
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
                        .child(MenuButton::new().child("Open").on_press(move |_| {
                            _ = input
                                .write()
                                .send(InputSignal::Command(Box::new(OpenFileCommand {})));
                        }))
                        .child(MenuButton::new().child("Save"))
                        .child(MenuButton::new().child("Close")),
                )
                .child(
                    SubMenu::new().label("Help").child(
                        MenuButton::new()
                            .on_press(move |_| {
                                _ = input
                                    .write()
                                    .send(InputSignal::Command(Box::new(ShowVersionCommand {})))
                            })
                            .child("Version"),
                    ),
                )
        }))
}
