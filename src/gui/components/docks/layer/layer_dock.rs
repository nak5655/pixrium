use crate::core::logics::commands::ChangeLayerOpacityCommand;
use crate::core::logics::Console;
use crate::gui::components::docks::layer::*;
use crate::FreyaServices;
use freya::prelude::*;

#[component]
pub fn LayerDock(
    console: Signal<Console<FreyaServices>>,
    opacity: Option<f32>,
    items: Option<Vec<LayerDockItemProps>>,
) -> Element {
    rsx! {
        rect {
            direction: "vertical",
            width: "fill",
            label {
                "Layers"
            },
            Slider {
                size: "80%",
                value: opacity.map(|opacity|  100.0 * opacity as f64).unwrap_or_default(),
                onmoved: move |p| {
                    console.write().execute(&ChangeLayerOpacityCommand {
                        opacity: (p * 0.01) as f32,
                    })
                }
            },
            ScrollView {
                direction: "vertical",
                width: "fill",
                height: "fill",
                for item in items.unwrap_or_default().iter() {
                    LayerDockItem {
                        name: item.name.clone()
                    }
                }
            }
        }
    }
}
