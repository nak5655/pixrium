use freya::prelude::*;

#[component]
pub fn MainMenu() -> Element {
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
                    label {
                        "Open"
                    }
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
