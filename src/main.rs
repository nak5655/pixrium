mod font;
mod tool;
mod widget;

use glam::{Vec3, vec2};
use iced::Length::Fill;
use iced::border::Radius;
use iced::event::Status;
use iced::widget::{Space, button, center, column, container, row, shader, text};
use iced::{Alignment, Background, Border, Color, Font, Length, Theme, alignment, window};
use iced::{Element, Task};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};
use iced_aw::{quad, widgets::InnerBounds};
use image::{self, ImageReader};
use rfd;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use tool::Tool;
use widget::sphere_canvas::sphere_canvas;

use crate::tool::ToolHandle;
use crate::tool::pan::PanMessage;
use crate::widget::sphere_canvas::{SphereCanvasMessage, SphereCanvasState};

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

    SphereCanvasMessage(widget::sphere_canvas::SphereCanvasMessage),

    ChangeTool(ToolHandle),

    Exit,
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
}

struct App {
    image_path: PathBuf,
    image: Arc<image::DynamicImage>,

    canvas_state: Arc<RwLock<SphereCanvasState>>,

    current_tool: ToolHandle,
    pan_tool: ToolHandle,
    zoom_tool: ToolHandle,
}

impl App {
    fn new() -> Self {
        let img = image::load_from_memory(SAMPLE_IMAGE_BYTES).unwrap();

        let pan_tool = tool::ToolHandle {
            handle: Arc::new(tool::pan::PanTool::new()),
        };

        Self {
            image_path: PathBuf::new(),
            image: Arc::new(img),
            canvas_state: Arc::new(RwLock::new(SphereCanvasState::default())),
            current_tool: pan_tool.clone(),
            pan_tool: pan_tool.clone(),
            zoom_tool: tool::ToolHandle {
                handle: Arc::new(tool::zoom::ZoomTool::new()),
            },
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

            Message::SphereCanvasMessage(msg) => {
                match msg {
                    widget::sphere_canvas::SphereCanvasMessage::MousePressed {
                        button,
                        position,
                    } => {
                        if let Ok(mut state) = self.canvas_state.write() {
                            state.mouse_button = Some(button);
                            state.mouse_delta = vec2(0.0, 0.0);
                        }

                        let mut status = self
                            .current_tool
                            .handle
                            .on_mouse_pressed(&self.canvas_state);
                        if status == Status::Ignored {
                            status = self.pan_tool.handle.on_mouse_pressed(&self.canvas_state);
                        }
                        if status == Status::Ignored {
                            status = self.zoom_tool.handle.on_mouse_pressed(&self.canvas_state);
                        }
                    }
                    widget::sphere_canvas::SphereCanvasMessage::MouseReleased {
                        button,
                        position,
                    } => {
                        if let Ok(mut state) = self.canvas_state.write() {
                            state.mouse_button = None;
                        }

                        let mut status = self
                            .current_tool
                            .handle
                            .on_mouse_released(&self.canvas_state);
                        if status == Status::Ignored {
                            status = self.pan_tool.handle.on_mouse_released(&self.canvas_state);
                        }
                        if status == Status::Ignored {
                            status = self.zoom_tool.handle.on_mouse_released(&self.canvas_state);
                        }
                    }
                    widget::sphere_canvas::SphereCanvasMessage::MouseMoved { position } => {
                        if let Ok(mut state) = self.canvas_state.write() {
                            state.mouse_delta = vec2(
                                position.x - state.mouse_point_prev.x,
                                position.y - state.mouse_point_prev.y,
                            );
                            state.mouse_point_prev = vec2(position.x, position.y);
                            state.mouse_point = position;
                        }

                        let mut status =
                            self.current_tool.handle.on_mouse_moved(&self.canvas_state);
                        if status == Status::Ignored {
                            status = self.pan_tool.handle.on_mouse_moved(&self.canvas_state);
                        }
                        if status == Status::Ignored {
                            status = self.zoom_tool.handle.on_mouse_moved(&self.canvas_state);
                        }
                    }
                    widget::sphere_canvas::SphereCanvasMessage::MouseWheel { delta } => {
                        if let Ok(mut state) = self.canvas_state.write() {
                            state.mouse_wheel_delta = delta;
                        }

                        let mut status = self.current_tool.handle.on_wheel(&self.canvas_state);
                        if status == Status::Ignored {
                            status = self.pan_tool.handle.on_wheel(&self.canvas_state);
                        }
                        if status == Status::Ignored {
                            status = self.zoom_tool.handle.on_wheel(&self.canvas_state);
                        }
                    }
                    widget::sphere_canvas::SphereCanvasMessage::BoundsChanged(_bounds) => {
                        if let Ok(mut write_state) = self.canvas_state.write() {
                            write_state.viewport_bounds = _bounds;
                        }
                    }
                }

                Task::none()
            }

            Message::ChangeTool(tool) => {
                self.current_tool = tool;
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
                    self.tool_button(&self.pan_tool),
                    self.tool_button(&self.zoom_tool),
                ]
                .height(Length::Fill),
                shader((|| {
                    sphere_canvas(
                        self.image.clone(),
                        self.canvas_state.clone(),
                    ).on_event(|msg| {
                        match msg {
                            SphereCanvasMessage::MousePressed { button, position } => Message::SphereCanvasMessage(SphereCanvasMessage::MousePressed { button, position }),
                            SphereCanvasMessage::MouseReleased { button, position } => Message::SphereCanvasMessage(SphereCanvasMessage::MouseReleased { button, position }),
                            SphereCanvasMessage::MouseMoved { position } => Message::SphereCanvasMessage(SphereCanvasMessage::MouseMoved { position }),
                            SphereCanvasMessage::MouseWheel { delta } => Message::SphereCanvasMessage(SphereCanvasMessage::MouseWheel { delta }),
                            SphereCanvasMessage::BoundsChanged(bounds) => Message::SphereCanvasMessage(SphereCanvasMessage::BoundsChanged(bounds)),
                        }
                    })
                })())
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .width(Length::Fill)
            .height(Length::Fill),
            row![
                container(text!("{}", self.image_path.as_path().to_str().unwrap()))
                    .width(Length::Fill),
                container(row![
                    (|| {
                        if let Ok(state) = self.canvas_state.read() {
                            let (x, y) = Self::look_at_to_latlng(state.look_at);
                            text!(
                                "N:{:.2}°, E:{:.2}°, FOV:{:.2}°",
                                Self::rad2degree(x),
                                Self::rad2degree(y),
                                Self::rad2degree(state.aov),
                            )
                        } else {
                            text!("N:--°, E:--°, FOV:--°")
                        }
                    })()
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

    fn menu_bar_item(
        label: &'_ str,
    ) -> container::Container<'_, Message, iced::Theme, iced::Renderer> {
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

    fn tool_button_style(
        &self,
        tool: ToolHandle,
    ) -> impl Fn(&Theme, button::Status) -> button::Style + '_ {
        move |_, status| {
            if status == button::Status::Pressed || self.current_tool == tool {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(60, 60, 200))),
                    text_color: Color::from_rgb8(255, 255, 255),
                    ..button::Style::default()
                }
            } else if status == button::Status::Hovered {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(120, 120, 220))),
                    text_color: Color::from_rgb8(255, 255, 255),
                    ..button::Style::default()
                }
            } else if status == button::Status::Active {
                button::Style {
                    background: None,
                    text_color: Color::from_rgb8(60, 60, 60),
                    ..button::Style::default()
                }
            } else {
                button::Style {
                    background: None,
                    text_color: Color::from_rgb8(200, 200, 200),
                    ..button::Style::default()
                }
            }
        }
    }

    fn tool_button(
        &self,
        tool: &ToolHandle,
    ) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
        button(text(tool.handle.icon()).font(font::icon_font()).size(32))
            .style(self.tool_button_style(tool.clone()))
            .on_press(Message::ChangeTool(tool.clone()))
    }

    fn rad2degree(rad: f32) -> f32 {
        rad * (180.0 / PI)
    }

    // Convert a look_at vector to latitude and longitude in radians
    fn look_at_to_latlng(look_at: Vec3) -> (f32, f32) {
        let x = look_at.z.atan2(look_at.x);
        let y = look_at
            .y
            .atan2((look_at.x.powi(2) + look_at.z.powi(2)).sqrt());

        (x, y)
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
