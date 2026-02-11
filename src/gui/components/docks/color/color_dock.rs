use crate::core::input::InputSignal;
use crate::FreyaServices;
use std::sync::mpsc::Sender;

use crate::gui::components::docks::Dock;
use freya::prelude::*;
use skia_safe::{colors, Color4f};

pub fn color_dock(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let mut color = use_state::<Color4f>(|| colors::BLACK);

    let r_slider = color_slider("R", color.read().r, move |value| {
        let r = value;
        let g = color.peek().g;
        let b = color.peek().b;
        let a = color.peek().a;
        color.set(Color4f::new(r, g, b, a));
        _ = input
            .write()
            .send(InputSignal::ChooseColor(color.peek().clone()));
    });

    let g_slider = color_slider("G", color.read().g, move |value| {
        let r = color.peek().r;
        let g = value;
        let b = color.peek().b;
        let a = color.peek().a;
        color.set(Color4f::new(r, g, b, a));
        _ = input
            .write()
            .send(InputSignal::ChooseColor(color.peek().clone()));
    });

    let b_slider = color_slider("B", color.read().b, move |value| {
        let r = color.peek().r;
        let g = color.peek().g;
        let b = value;
        let a = color.peek().a;
        color.set(Color4f::new(r, g, b, a));
        _ = input
            .write()
            .send(InputSignal::ChooseColor(color.peek().clone()));
    });

    let a_slider = color_slider("A", color.read().a, move |value| {
        let r = color.peek().r;
        let g = color.peek().g;
        let b = color.peek().b;
        let a = value;
        color.set(Color4f::new(r, g, b, a));
        _ = input
            .write()
            .send(InputSignal::ChooseColor(color.peek().clone()));
    });

    Dock::new().title("Color").child(
        rect()
            .direction(Direction::Vertical)
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::px(20.))
                    .background((
                        (color.read().r * 255.) as u8,
                        (color.read().g * 255.) as u8,
                        (color.read().b * 255.) as u8,
                        color.read().a,
                    )),
            )
            .child(r_slider)
            .child(g_slider)
            .child(b_slider)
            .child(a_slider),
    )
}

fn color_slider(
    label: &str,
    value: f32,
    mut on_move: impl FnMut(f32) + 'static,
) -> impl IntoElement {
    rect()
        .direction(Direction::Horizontal)
        .child(rect().child(label).width(Size::percent(5.)))
        .child(
            rect()
                .child(
                    Slider::new(move |per| on_move((per / 100.) as f32)).value(value as f64 * 100.),
                )
                .padding((0., 10., 0., 10.))
                .width(Size::percent(75.)),
        )
        .child(format!("{:.2}", value))
}
