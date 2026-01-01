use crate::core::logics::Console;
use crate::gui::components::canvas::{CanvasState, CanvasView};
use crate::gui::components::docks::layer::{LayerDock, LayerDockItemProps};
use crate::gui::windows::main::main_menu::MainMenu;
use crate::FreyaServices;
use freya::prelude::*;

#[component]
pub fn main_window(mut console: Signal<Console<FreyaServices>>, mut canvas_state: Signal<CanvasState>) -> Element {
    let layer_opacity = use_memo(move || {
        console
            .read()
            .session
            .project
            .as_ref()
            .map(|project| match project.layers.first() {
                Some(layer) => layer.opacity,
                None => 0.0,
            })
    });

    let layer_items = use_memo(move || {
        console.read().session.project.as_ref().map(|project| {
            project
                .layers
                .iter()
                .map(|layer| LayerDockItemProps {
                    name: layer.name.clone(),
                })
                .collect()
        })
    });

    let (_, size) = use_node_signal();

    rsx! {
        Body {
            direction: "vertical",
            width: "fill",
            height: "fill",
            MainMenu { console },
            rect {
                width: "fill",
                height: "90%",
                ResizableContainer {
                    direction: "horizontal",
                    ResizablePanel {
                        initial_size: 70.0,
                        CanvasView {
                            console,
                            canvas_state,
                        }
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
                },
            },
            rect {
                direction: "horizontal",
                height: "auto",
                width: "fill",
                label {
                    { format!("{:.2}, {:.2}, {:.2}"
                        , canvas_state.read().look_at.x
                        , canvas_state.read().look_at.y
                        , canvas_state.read().look_at.z) }
                },
                label {
                    { format!("{:.2}", canvas_state.read().fov.0) }
                },
            }
        }
    }
}
