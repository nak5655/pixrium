use crate::gui::components::ToolbarState;

use crate::gui::components::canvas::CanvasState;

pub struct MainState {
    pub canvas: Option<CanvasState>,
    pub toolbar: ToolbarState,
}

impl MainState {
    pub fn new(canvas: Option<CanvasState>, toolbar: ToolbarState) -> Self {
        Self { canvas, toolbar }
    }
}
