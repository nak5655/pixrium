use glam::Vec3;
use skia_safe::Point;
use crate::core::data::config::CommandId;
use crate::core::data::Project;
use crate::core::inputs::{Key, KeyboardInput};
use crate::core::inputs::PointerInput;
use crate::core::logics::tools::{EventHandling, PanTool, Tool};
use crate::core::math::Radian;
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
        EventHandling::None
    }

    fn keyboard_input(&mut self, input: &KeyboardInput, project: &mut Project, services: &mut S) -> EventHandling {
        match input {
            KeyboardInput::Down { key } => {
                if services.config().get_key_binding(CommandId::ZoomIn) == *key {
                    let fov = services.canvas().fov() * 0.9;
                    services.canvas_mut().zoom(Radian(fov));
                    EventHandling::Captured
                }
                else if services.config().get_key_binding(CommandId::ZoomOut) == *key {
                    let fov = services.canvas().fov() * 1.1;
                    services.canvas_mut().zoom(Radian(fov));
                    EventHandling::Captured
                }
                else {
                    EventHandling::None
                }
            }
            _ => EventHandling::None
        }
    }
}