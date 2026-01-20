use crate::core::logics::commands::ChangeLayerOpacityCommand;
use crate::core::logics::Console;
use crate::gui::components::docks::layer::LayerDockItem;
use crate::FreyaServices;
use freya::prelude::*;

pub fn layer_dock(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    rect()
        .direction(Direction::Vertical)
        .width(Size::fill())
        .padding(2.)
        .child(
            rect()
                .width(Size::fill())
                .padding((2., 4.))
                .background((224, 224, 224))
                .child("Layers"),
        )
        .child(
            rect()
                .direction(Direction::Horizontal)
                .width(Size::fill())
                .height(Size::auto())
                .padding((4., 0., 0., 0.))
                .child("Opacity")
                .child(
                    rect()
                        .width(Size::fill())
                        .height(Size::auto())
                        .padding((0., 8.))
                        .child(
                            Slider::new(move |p| {
                                console.write().execute(&ChangeLayerOpacityCommand {
                                    opacity: (p * 0.01) as f32,
                                })
                            })
                            .size(Size::fill())
                            .value(
                                console
                                    .read()
                                    .session
                                    .selected_layer()
                                    .map(|layer| 100.0 * layer.opacity as f64)
                                    .unwrap_or_default(),
                            ),
                        ),
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
