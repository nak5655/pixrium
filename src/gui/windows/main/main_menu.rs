use crate::core::data::Session;
use crate::core::logics::commands::Commands;
use crate::core::logics::Console;
use crate::FreyaServices;
use freya::prelude::*;

#[component]
pub fn MainMenu(mut props: MainMenuProps) -> Element {
    let mut show_menu = use_signal(|| false);

    rsx!(
        Button {
            onpress: move |_| show_menu.toggle(),
            label { "Menu" }
        },
        if *show_menu.read() {
            Menu {
                onclose: move |_| show_menu.set(false),
                SubMenu {
                    menu: rsx!(
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
                        MenuButton {
                            label {
                                "Close"
                            }
                        }
                    ),
                    label {
                        "File"
                    },
                }
                SubMenu {
                    menu: rsx!(
                        MenuButton {
                            onpress: move |_| props.console.write().run(Commands::ShowVersion),
                            label {
                                "Version"
                            }
                        }
                    ),
                    label {
                        "Help"
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
