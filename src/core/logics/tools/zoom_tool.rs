use glam::Vec3;
use skia_safe::Point;
use crate::core::data::Project;
use crate::core::inputs::pointer_input::PointerInput;
use crate::core::logics::tools::{EventHandling, PanTool, Tool};
use crate::core::services::{CanvasService, Services};

pub struct ZoomTool {
}

impl ZoomTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl <S: Services> Tool<S> for ZoomTool {
    fn pointer_input(&mut self, input: &PointerInput, project: &mut Project, services: &mut S) -> EventHandling {
        match input {
            PointerInput::ScrollY { delta, viewport_position, uv_position } => {
                EventHandling::Captured
            }
            _ => EventHandling::None
        }
    }
}