mod brush_tool;
mod pan_tool;
mod zoom_tool;

use crate::core::data::Session;
use crate::core::inputs::{KeyboardInput, PointerInput};
use crate::core::services::Services;
pub use brush_tool::*;
pub use pan_tool::*;
pub use zoom_tool::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Tools {
    Pan,
    Brush,
    Zoom,
}

pub trait Tool<S>
where
    S: Services,
{
    fn pointer_input(
        &mut self,
        input: &PointerInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling;
    fn keyboard_input(
        &mut self,
        input: &KeyboardInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling;
}

#[derive(Copy, Clone, Debug)]
pub enum EventHandling {
    Ignored,
    Captured,
}
