use std::sync::{Arc, Mutex, RwLock};
use freya::prelude::*;
use skia_safe::Bitmap;
use crate::core::data::Session;
use crate::core::data::Layer;
use crate::core::data::Project;
use crate::core::logics::Console;
use crate::core::services::Services;
use crate::FreyaServices;
use crate::gui::components::canvas::Canvas;
use crate::gui::components::docks::layer::{LayerDockProps, LayerDock, LayerDockItemProps};
use crate::gui::services::RfdFileDialogService;
use crate::gui::windows::main::main_menu::MainMenu;

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
