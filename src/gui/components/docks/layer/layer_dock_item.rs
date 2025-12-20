use crate::core::data::Layer;
use freya::prelude::*;

#[component]
pub fn LayerDockItem(props: LayerDockItemProps) -> Element {
    rsx!(
        label {
            { props.name }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct LayerDockItemProps {
    name: String,
}

impl LayerDockItemProps {
    pub fn new(layer: &Layer) -> Self {
        Self {
            name: layer.name.clone(),
        }
    }
}
