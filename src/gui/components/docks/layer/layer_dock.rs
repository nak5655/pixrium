use crate::core::logics::commands::ChangeLayerOpacityCommand;
use crate::core::logics::Console;
use crate::gui::components::docks::layer::LayerDockItem;
use crate::FreyaServices;
use freya::prelude::*;

pub fn layer_dock(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    rect()
        .direction(Direction::Vertical)
        .width(Size::fill())
        .child("Layers")
        .child(
            Slider::new(move |p| {
                console.write().execute(&ChangeLayerOpacityCommand {
                    opacity: (p * 0.01) as f32,
                })
            })
            .size(Size::percent(80.))
            .value(
                console
                    .read()
                    .session
                    .selected_layer()
                    .map(|layer| 100.0 * layer.opacity as f64)
                    .unwrap_or_default(),
            ),
        )
        .child(
            ScrollView::new()
                .direction(Direction::Vertical)
                .width(Size::fill())
                .height(Size::fill())
                .children_iter(
                    console
                        .read()
                        .session
                        .layers()
                        .iter()
                        .map(|item| LayerDockItem::new(item.name.clone()).into()),
                ),
        )
}
