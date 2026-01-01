mod pan_tool;

pub use pan_tool::*;
use crate::core::data::{Project, Session};
use crate::core::inputs::pointer_input::*;
use crate::core::services::Services;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Tools {
    Pan,
}

pub trait Tool<S>
where S: Services {
    fn pointer_input(&mut self, input: &PointerInput, project: &mut Project, services: &mut S) -> EventHandling;
}

#[derive(Copy, Clone, Debug)]
pub enum EventHandling {
    None,
    Captured,
}