use freya::prelude::*;
use crate::core::data::layer::Layer;

#[component]
pub fn LayerDetailDock(props: LayerDetailProps) -> Element {
    rsx!(
        Slider {
            size: "80%",
            value: 100.0 * props.opacity as f64,
            onmoved: move |p| {

            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct LayerDetailProps {
    opacity: f32,
}

impl LayerDetailProps {
    pub fn new(layer: &Layer) -> Self {
        Self {
            opacity: layer.opacity,
        }
    }
}