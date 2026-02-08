use crate::core::data::{LayerState, ViewportState};
use crate::core::logics::tools::Tools;

#[derive(PartialEq, Clone)]
pub struct SessionState {
    pub active_tool: Tools,
    pub selected_layer_index: usize,
    pub viewport: ViewportState,
    pub layers: Vec<LayerState>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            active_tool: Tools::Pan,
            selected_layer_index: 0,
            viewport: ViewportState::new(),
            layers: vec![],
        }
    }
}
