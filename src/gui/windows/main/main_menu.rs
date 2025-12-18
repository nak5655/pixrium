use freya::prelude::*;
use crate::core::data::Session;
use crate::{FreyaServices};
use crate::core::logics::{Commands, Console};

#[component]
pub fn MainMenu(mut props: MainMenuProps) -> Element {
    let mut show_menu = use_signal(|| false);

    rsx!(
        Button {
            onpress: move |_| show_menu.toggle(),
            label { "Open Menu" }
        },
        if *show_menu.read() {
            Menu {
                onclose: move |_| show_menu.set(false),
                MenuButton {
                    onpress: move |_| props.console.write().run(Commands::OpenFile),
                    label {
                        "Open"
                    },
                }
                MenuButton {
                    label {
                        "Save"
                    }
                }
                SubMenu {
                    menu: rsx!(
                        MenuButton {
                            label {
                                "Some option"
                            }
                        }
                    ),
                    label {
                        "Options"
                    }
                }
                MenuButton {
                    label {
                        "Close"
                    }
                }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct MainMenuProps {
    pub console: Signal<Console<FreyaServices>>,
}
