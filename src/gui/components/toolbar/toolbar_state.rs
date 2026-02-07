use crate::core::logics::tools::Tools;

#[derive(PartialEq, Clone)]
pub struct ToolbarState {
    pub active_tool: Tools,
}

impl ToolbarState {
    pub fn new() -> Self {
        Self {
            active_tool: Default::default(),
        }
    }
}
