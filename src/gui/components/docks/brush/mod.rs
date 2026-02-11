use crate::core::input::InputSignal;
use crate::gui::components::docks::Dock;
use crate::FreyaServices;
use freya::prelude::*;
use std::sync::mpsc::Sender;
pub fn brush_dock(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let mut width = use_state(|| 3.0);

    Dock::new()
        .title("Brush")
        .child(size_slider("Width", *width.read(), move |w| {
            width.set(w);
            input.write().send(InputSignal::ChangeBrushWidth(w));
        }))
}

fn size_slider(
    label: &str,
    value: f32,
    mut on_move: impl FnMut(f32) + 'static,
) -> impl IntoElement {
    rect()
        .direction(Direction::Horizontal)
        .child(rect().child(label).width(Size::percent(25.)))
        .child(
            rect()
                .child(Slider::new(move |per| on_move(per as f32)).value(value as f64))
                .padding((0., 10., 0., 10.))
                .width(Size::percent(55.)),
        )
        .child(format!("{:.2}", value))
}
