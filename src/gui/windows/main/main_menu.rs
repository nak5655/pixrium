use crate::core::logics::commands::{OpenFileCommand, ShowVersionCommand};
use crate::core::logics::Console;
use crate::FreyaServices;
use freya::prelude::*;

#[component]
pub fn MainMenu(
    console: Signal<Console<FreyaServices>>
) -> Element {
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
                            onpress: move |_| {
                                console.write().execute(&OpenFileCommand { })
                            },
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
                            onpress: move |_| {
                                console.write().execute(&ShowVersionCommand { })
                            },
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

