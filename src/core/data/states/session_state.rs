use crate::core::data::ViewportState;
use crate::core::logics::tools::Tools;

#[derive(PartialEq, Copy, Clone)]
pub struct SessionState {
    pub active_tool: Tools,
    pub selected_layer_index: usize,
    pub viewport: ViewportState,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            active_tool: Tools::Brush,
            selected_layer_index: 0,
            viewport: ViewportState::new(),
        }
    }
}
