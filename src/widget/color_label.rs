use iced::mouse;
use iced::widget::canvas::{self, Cache, Fill, Geometry, Path};
use iced::{Color, Rectangle};
use iced::{Renderer, Theme};
use image::Rgba;
use std::vec;

pub struct ColorLabel {
    pub rgba: Rgba<u8>,
    cache: Cache,
}

impl ColorLabel {
    pub fn new(rgba: Rgba<u8>) -> Self {
        Self {
            rgba,
            cache: Cache::default(),
        }
    }
}

impl<Message> canvas::Program<Message> for ColorLabel {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let color = Color::from_rgba8(
                self.rgba[0],
                self.rgba[1],
                self.rgba[2],
                self.rgba[3] as f32 / 255.0,
            );

            let rect = Path::rectangle(iced::Point::ORIGIN, bounds.size());
            frame.fill(&rect, Fill::from(color))
        });

        vec![geometry]
    }
}
