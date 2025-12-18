use freya::prelude::*;
use crate::core::data::Session;
use crate::core::data::Layer;
use crate::gui::components::docks::layer::*;

#[component]
pub fn LayerDock(props: LayerDockProps) -> Element {
    rsx!(
        rect {
            direction: "vertical",
            label {
                "Layers"
            },
            Slider {
                size: "80%",
                value: props.opacity.map(|opacity|  100.0 * opacity as f64).unwrap_or_default(),
                onmoved: move |p| {

                }
            },
            ScrollView {
                direction: "vertical",
                width: "fill",
                height: "fill",
                for item in props.items.unwrap_or_default() {
                    LayerDockItem { ..item }
                }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct LayerDockProps {
    opacity: Option<f32>,
    items: Option<Vec<LayerDockItemProps>>,
}

impl LayerDockProps {
    pub fn new(session: &Session) -> Self {
        Self {
            opacity: session.project.as_ref().and_then(|project| {
                project.layers.first().map(|layer| {
                    layer.opacity
                })
            }),
            items: session.project.as_ref().map(|project| {
                project.layers.iter().map(|layer|
                    LayerDockItemProps::new(layer)
                ).collect()
            }),
        }
    }
}
