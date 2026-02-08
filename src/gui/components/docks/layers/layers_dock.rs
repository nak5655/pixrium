use crate::core::input::commands::ChangeLayerOpacityCommand;
use crate::core::input::InputSignal;
use crate::gui::components::docks::layers::LayersDockItem;
use crate::gui::windows::main::MainState;
use crate::{FreyaServices, OutputChannel};
use freya::prelude::*;
use freya_radio::hooks::use_radio;
use std::sync::mpsc::Sender;

pub fn layers_dock(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let layers_radio = use_radio::<MainState, OutputChannel>(OutputChannel::Layers);

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
        .maybe_child(layers_radio.read().layers.as_ref().map(|layers| {
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
                                _ = input.write().send(InputSignal::Command(Box::new(
                                    ChangeLayerOpacityCommand {
                                        opacity: (p * 0.01) as f32,
                                    },
                                )));
                            })
                            .size(Size::fill())
                            .value(
                                layers
                                    .selected_layer
                                    .as_ref()
                                    .map(|layer| 100.0 * layer.opacity as f64)
                                    .unwrap_or_default(),
                            ),
                        ),
                )
        }))
        .maybe_child(layers_radio.read().layers.as_ref().map(|layers| {
            ScrollView::new()
                .direction(Direction::Vertical)
                .width(Size::fill())
                .height(Size::fill())
                .children(
                    layers
                        .layers
                        .iter()
                        .map(|item| LayersDockItem::new(item.name.clone()).into()),
                )
        }))
}
