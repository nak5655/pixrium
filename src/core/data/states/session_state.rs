use crate::core::data::{LayerState, ViewportState};
use crate::core::logics::tools::Tools;
use skia_safe::{colors, Color4f};

#[derive(PartialEq, Clone)]
pub struct SessionState {
    pub active_tool: Tools,
    pub brush_width: f32,
    pub color: Color4f,
    pub selected_layer_index: usize,
    pub viewport: ViewportState,
    pub layers: Vec<LayerState>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            active_tool: Tools::Pan,
            brush_width: 3.0,
            color: colors::BLACK,
            selected_layer_index: 0,
            viewport: ViewportState::new(),
            layers: vec![],
        }
    }
}
