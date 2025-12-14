use freya::prelude::*;
use crate::core::data::layer::Layer;

#[component]
pub fn LayerTreeDock(items: Vec<LayerTreeItemProps>) -> Element {
    rsx!(
        rect {
            direction: "vertical",
            label {
                "Layers"
            },
            ScrollView {
                direction: "vertical",
                width: "fill",
                height: "fill",
                for item in items {
                    LayerTreeItem { ..item }
                }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct LayerTreeItemProps {
    name: String,
}

impl LayerTreeItemProps {
    pub fn new(layer: &Layer) -> Self {
        Self {
            name: layer.name.clone(),
        }
    }
}

#[component]
fn LayerTreeItem(props: LayerTreeItemProps) -> Element {
    rsx!(
        label {
            { props.name }
        }
    )
}