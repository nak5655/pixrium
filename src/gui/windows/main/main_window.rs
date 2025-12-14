use std::sync::{Arc, Mutex, RwLock};
use freya::prelude::*;
use skia_safe::Bitmap;
use crate::core::data::layer::Layer;
use crate::core::data::project::Project;
use crate::gui::components::canvas::Canvas;
use crate::gui::components::docks::layer_detail_dock::LayerDetailDock;
use crate::gui::components::docks::layer_tree_dock::{LayerTreeItemProps, LayerTreeDock};
use crate::gui::windows::main::main_menu::MainMenu;

#[component]
pub fn MainWindow() -> Element {
    let mut project = Project::new(4000, 2000);
    project.add_layer(String::from("Layer 1"));

    rsx!(
        Body {
            direction: "vertical",
            width: "fill",
            height: "fill",
            MainMenu { },
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
                        LayerDetailDock {
                            opacity: { project.layers.first().unwrap().opacity }
                        },
                        LayerTreeDock {
                            items: { project.layers.iter().map(|layer| LayerTreeItemProps::new(&layer)).collect() }
                        }
                    }
                }
            }
        }
    )
}