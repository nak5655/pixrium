use crate::core::data::config::CommandId;
use crate::core::data::Session;
use crate::core::inputs::KeyboardInput;
use crate::core::inputs::PointerInput;
use crate::core::logics::tools::{EventHandling, Tool};
use crate::core::math::Radian;
use crate::core::services::Services;

pub struct ZoomTool {}

impl ZoomTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: Services> Tool<S> for ZoomTool {
    fn pointer_input(
        &mut self,
        input: &PointerInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling {
        match input {
            PointerInput::Scroll {
                delta,
                viewport_position,
            } => {
                let fov = session.fov() * if delta.y < 0.0 { 1.1 } else { 0.9 };
                session.zoom(Radian(fov));
                EventHandling::Captured
            }
            _ => EventHandling::Ignored,
        }
    }

    fn keyboard_input(
        &mut self,
        input: &KeyboardInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling {
        match input {
            KeyboardInput::Down { key } => {
                if services.config().get_key_binding(CommandId::ZoomIn) == *key {
                    let fov = session.fov() * 0.9;
                    session.zoom(Radian(fov));
                    EventHandling::Captured
                } else if services.config().get_key_binding(CommandId::ZoomOut) == *key {
                    let fov = session.fov() * 1.1;
                    session.zoom(Radian(fov));
                    EventHandling::Captured
                } else {
                    EventHandling::Ignored
                }
            }
            _ => EventHandling::Ignored,
        }
    }
}
