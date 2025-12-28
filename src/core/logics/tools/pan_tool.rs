use skia_safe::Point;
use crate::core::data::Project;
use crate::core::inputs::pointer_input::PointerInput;
use crate::core::logics::tools::Tool;

pub struct PanTool {
    is_dragging: bool,
    drag_start: Point,
}

impl PanTool {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            drag_start: Point::default(),
        }
    }
}

impl Tool for PanTool {
    fn pointer_input(&mut self, project: &mut Project, input: &PointerInput) {
        match input {
            PointerInput::Down { button, viewport_position, uv_position } => {
                self.is_dragging = true;
                self.drag_start = *viewport_position;
            }
            PointerInput::Move { button, viewport_position, uv_position } => {

            }
            PointerInput::Up { button, viewport_position, uv_position } => {
                self.is_dragging = false;
            }
        }
    }
}