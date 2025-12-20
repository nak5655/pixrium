use crate::core::data::Layer;
use crate::core::logics::Console;
use crate::gui::components::canvas::Canvas;
use crate::gui::components::docks::layer::{LayerDock, LayerDockItemProps, LayerDockProps};
use crate::gui::windows::main::main_menu::MainMenu;
use crate::FreyaServices;
use freya::prelude::*;
use skia_safe::Bitmap;
use std::sync::{Arc, Mutex, RwLock};

#[component]
pub fn MainWindow(props: MainWindowProps) -> Element {
    rsx! {
        Body {
            direction: "vertical",
            width: "fill",
            height: "fill",
            MainMenu { console: props.console },
            ResizableContainer {
                direction: "horizontal",
                ResizablePanel {
                    initial_size: 70.0,
                    Canvas { }
                },
                ResizableHandle { },
                ResizablePanel {
                    initial_size: 30.0,
                    rect {
                        direction: "vertical",
                        LayerDock {
                            ..{ LayerDockProps::new(&props.console.read().session) }
                        },
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct MainWindowProps {
    pub console: Signal<Console<FreyaServices>>,
}
