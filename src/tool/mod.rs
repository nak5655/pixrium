use core::fmt;
use iced::advanced::graphics::core::event::Status;
use std::sync::Arc;
use std::sync::RwLock;

use crate::widget::sphere_canvas::SphereCanvasState;

pub mod pan;
pub mod pen;
pub mod zoom;

pub trait Tool {
    fn name(&self) -> &str;
    fn icon(&self) -> char;

    fn on_mouse_pressed(&self, _canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_mouse_released(&self, _canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_mouse_moved(&self, _canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        Status::Ignored
    }

    fn on_wheel(&self, _canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
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
