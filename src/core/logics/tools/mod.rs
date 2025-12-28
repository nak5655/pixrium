mod pan_tool;

pub use pan_tool::*;
use crate::core::data::{Project, Session};
use crate::core::inputs::pointer_input::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Tools {
    Pan,
}

pub trait Tool {
    fn pointer_input(&mut self, project: &mut Project, input: &PointerInput);
}
