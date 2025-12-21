use crate::core::logics::Console;
use crate::gui::components::canvas::Canvas;
use crate::gui::components::docks::layer::{LayerDock, LayerDockItemProps};
use crate::gui::windows::main::main_menu::MainMenu;
use crate::FreyaServices;
use freya::prelude::*;

#[component]
pub fn MainWindow(console: Signal<Console<FreyaServices>>) -> Element {
    let layer_opacity = use_memo(move || {
        console.read().session.project.as_ref().map(|project| {
            match project.layers.first() {
                Some(layer) => layer.opacity,
                None => 0.0
            }
        })
    });

    let layer_items = use_memo(move || {
        console.read().session.project.as_ref().map(|project| {
            project.layers.iter().map(|layer|
                LayerDockItemProps {
                    name: layer.name.clone()
                }
            ).collect()
        })
    });

    rsx! {
        Body {
            direction: "vertical",
            width: "fill",
            height: "fill",
            MainMenu { console },
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
                        width: "fill",
                        LayerDock {
                            console,
                            opacity: layer_opacity.read().clone(),
                            items: layer_items.read().clone(),
                        },
                    }
                }
            }
        }
    }
}
