use core::fmt;
use glam::Vec2;
use glam::vec2;
use iced::advanced::Shell;
use iced::advanced::graphics::core::event::Status;
use iced::advanced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::shader;
use iced::{Rectangle, mouse};
use std::any::Any;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::widget::sphere_canvas::SphereCanvasState;

pub mod pan;
pub mod zoom;

pub trait ToolMessage: Any + Clone {}

pub trait Tool {
    fn name(&self) -> &str;
    fn icon(&self) -> char;

    fn on_mouse_pressed(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_mouse_released(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_mouse_moved(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_wheel(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }
}

#[derive(Clone)]
pub struct ToolHandle {
    pub handle: Arc<dyn Tool + Send + Sync>,
}

impl PartialEq for ToolHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.handle, &other.handle)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl fmt::Debug for ToolHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.handle.name())
    }
}
