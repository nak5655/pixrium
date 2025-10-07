use std::sync::Arc;

use iced::advanced::graphics::core::event::Status;

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
