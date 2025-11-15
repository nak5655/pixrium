use iced::{
    Color, Element, Length, Rectangle, Renderer, Size,
    advanced::{
        Layout, Widget, layout, mouse, renderer,
        widget::tree::{State, Tree},
    },
    widget::{center, column, row, slider, text},
};
use image::Rgba;
use std::sync::Arc;

pub struct ColorDock<'a, Message>
where
    Message: Clone,
{
    on_color_changed: Box<dyn Fn(Rgba<u8>) -> Message + 'a>,
}

impl<'a, Message> ColorDock<'a, Message>
where
    Message: Clone,
{
    pub fn new(on_color_changed: impl 'static + Fn(Rgba<u8>) -> Message + 'a) -> Self
    where
        Message: Clone,
    {
        let color_dock = ColorDock {
            on_color_changed: Box::new(on_color_changed),
        };

        color_dock
    }

    pub fn view(&self, color: Rgba<u8>) -> Element<'_, Message> {
        column![
            row![
                text!("R"),
                slider(0u8..=255u8, color[0], move |r| {
                    (self.on_color_changed.as_ref())(Rgba([r, color[1], color[2], color[3]]))
                })
                .step(1),
            ],
            row![
                text!("G"),
                slider(0u8..=255u8, color[1], move |g| {
                    (self.on_color_changed.as_ref())(Rgba([color[0], g, color[2], color[3]]))
                })
                .step(1),
            ],
            row![
                text!("B"),
                slider(0u8..=255u8, color[2], move |b| {
                    (self.on_color_changed.as_ref())(Rgba([color[0], color[1], b, color[3]]))
                })
                .step(1),
            ],
        ]
        .into()
    }
}
