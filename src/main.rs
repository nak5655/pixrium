mod font;
mod widget;

use glam::vec2;
use iced::border::Radius;
use iced::widget::{button, center, column, container, row, shader, text, Space};
use iced::{
    alignment, window, Alignment, Background, Border, Color, Font, Length, Theme,
};
use iced::{Element, Task};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};
use iced_aw::{quad, widgets::InnerBounds};
use image::{self, ImageReader};
use rfd;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::Arc;
use widget::sphere_canvas::sphere_canvas;

#[cfg(windows)]
const SAMPLE_IMAGE_BYTES: &[u8] = include_bytes!("..\\resources\\images\\sample.png");
#[cfg(unix)]
const SAMPLE_IMAGE_BYTES: &[u8] = include_bytes!("../resources/images/sample.png");

fn main() -> iced::Result {
    iced::application("Pixrium", App::update, App::view)
        .font(font::UI_FONT_BYTES)
        .font(font::MONO_FONT_BYTES)
        .font(font::ICON_FONT_BYTES)
        .default_font(Font::with_name(font::FONT_NAME))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    FileOpened(Result<PathBuf, Error>),

    SphereCanvasAovChanged(f32),
    SphereCanvasLookAtChanged(glam::Vec2),

    Exit,
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
}

struct App {
    image_path: PathBuf,
    image: Arc<image::DynamicImage>,
    aov: f32,
    look_at: glam::Vec2,
}

impl App {
    fn new() -> Self {
        let img = image::load_from_memory(SAMPLE_IMAGE_BYTES).unwrap();

        Self {
            image_path: PathBuf::new(),
            image: Arc::new(img),
            aov: 1.0,
            look_at: vec2(0.0, 0.0),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile => Task::perform(open_file(), Message::FileOpened),
            Message::FileOpened(result) => {
                let image_path = result.unwrap();
                let dyn_image = ImageReader::open(image_path.as_path())
                    .expect("Failed to open image.")
                    .decode()
                    .expect("Failed to decode image.");
                self.image_path = image_path;
                self.image = Arc::new(dyn_image);

                Task::none()
            }
            Message::Exit => window::get_latest().and_then(window::close),

            Message::SphereCanvasAovChanged(aov) => {
                self.aov = aov;
                Task::none()
            }
            Message::SphereCanvasLookAtChanged(look_at) => {
                self.look_at = look_at;
                Task::none()
            }
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        let menu_tpl = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);

        #[rustfmt::skip]
        let content = column![
            menu_bar!(
                (Self::menu_bar_item("File"), menu_tpl(
                    menu_items!(
                        (Self::menu_button("Open").on_press(Message::OpenFile))
                        (Self::menu_button("Save"))
                        (Self::separator())
                        (Self::menu_button("Exit").on_press(Message::Exit))
                    )
                ))
                (Self::menu_bar_item("Edit"), menu_tpl(
                    menu_items!(
                        (Self::menu_button("Undo"))
                        (Self::menu_button("Redo"))
                        (Self::separator())
                        (Self::menu_button("Cut"))
                        (Self::menu_button("Copy"))
                        (Self::menu_button("Paste"))
                    )
                ))
            ),
            row![
                column![
                    // pen
                    Self::tool_button(&'\u{eb04}').on_press(Message::Exit),
                    // eraser
                    Self::tool_button(&'\u{eb8b}'),
                ],
                shader(sphere_canvas(
                    self.image.clone(),
                    Message::SphereCanvasAovChanged,
                    Message::SphereCanvasLookAtChanged
                ))
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .width(Length::Fill)
            .height(Length::Fill),
            row![
                container(text!("{}", self.image_path.as_path().to_str().unwrap()))
                    .width(Length::Fill),
                container(row![
                    text!(
                        "N:{:.2}°, E:{:.2}°, FOV:{:.2}°",
                        Self::rad2degree(self.look_at.x),
                        Self::rad2degree(self.look_at.y),
                        Self::rad2degree(self.aov),
                    )
                    .font(font::mono_font()),
                    Space::with_width(10)
                ])
                .align_x(alignment::Alignment::End)
            ]
        ];

        center(content).into()
    }

    fn menu_button_style(theme: &Theme, status: button::Status) -> button::Style {
        let base = button::Style {
            background: None,
            text_color: theme.palette().text,
            ..button::Style::default()
        };

        match status {
            button::Status::Active | button::Status::Pressed => base,
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(
                    theme.extended_palette().primary.base.color,
                )),
                ..base
            },
            button::Status::Disabled => button::Style {
                text_color: base.text_color.scale_alpha(0.5),
                ..base
            },
        }
    }

    fn menu_bar_item(label: &'_ str) -> container::Container<'_, Message, iced::Theme, iced::Renderer> {
        container(text(label).align_x(Alignment::Start)).padding([4, 8])
    }

    fn menu_button(label: &'_ str) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
        button(text(label).align_x(Alignment::Start))
            .padding([4, 8])
            .style(Self::menu_button_style)
            .width(Length::Fill)
    }

    fn separator() -> quad::Quad {
        quad::Quad {
            quad_color: Color::from([0.8; 3]).into(),
            quad_border: Border {
                radius: Radius::new(1.0),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(0.98, 0.2),
            height: Length::Fixed(2.0),
            ..Default::default()
        }
    }

    fn tool_button_style(_: &Theme, status: button::Status) -> button::Style {
        match status
        {
            button::Status::Pressed => button::Style {
                background: Some(Background::Color(Color::from_rgb8(60, 60, 200))),
                text_color: Color::from_rgb8(255, 255, 255),
                ..button::Style::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::from_rgb8(120, 120, 220))),
                text_color: Color::from_rgb8(255, 255, 255),
                ..button::Style::default()
            },
            button::Status::Active => button::Style {
                background: None,
                text_color: Color::from_rgb8(60, 60, 60),
                ..button::Style::default()
            },
            button::Status::Disabled => button::Style {
                background: None,
                text_color: Color::from_rgb8(200, 200, 200),
                ..button::Style::default()
            },
        }
    }

    fn tool_button(icon_codepoint: &'_ char) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
        button(text(icon_codepoint).font(font::icon_font()).size(32))
            .style(Self::tool_button_style)
    }

    fn rad2degree(rad: f32) -> f32 {
        rad * (180.0 / PI)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

async fn open_file() -> Result<PathBuf, Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(picked_file.into())
}
