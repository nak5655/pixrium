use std::sync::Arc;

use core::fmt;
use glam::{Quat, Vec2, Vec3, vec2, vec3};
use iced::advanced::Shell;
use iced::advanced::graphics::core::event;
use iced::advanced::graphics::core::event::Status;
use iced::advanced::mouse::Cursor;
use iced::mouse::{Button, ScrollDelta};
use iced::widget::shader;
use iced::widget::shader::wgpu;
use iced::{Rectangle, mouse};

use crate::tool::Tool;
use crate::widget::sphere_canvas::SphereCanvasState;

#[derive(Debug)]
pub struct ZoomTool {
    pub name: String,
    pub icon: char,
}

impl ZoomTool {
    pub fn new() -> Self {
        Self {
            name: "Zoom".to_string(),
            icon: '\u{fdaa}',
        }
    }
}

#[derive(Debug, Clone)]
pub enum ZoomMessage {
    AovChanged(f32),
}

impl Tool for ZoomTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn icon(&self) -> char {
        self.icon
    }

    fn on_wheel(&self, canvas_state: &Arc<std::sync::RwLock<SphereCanvasState>>) -> Status {
        if let Ok(mut canvas_state) = canvas_state.try_write() {
            canvas_state.aov = canvas_state.aov - canvas_state.mouse_wheel_delta / 10.0;
            return Status::Captured;
        };
        Status::Ignored
    }
}
