use crate::gui::components::ToolbarState;

use crate::gui::components::canvas::CanvasState;
use crate::gui::components::docks::layers::LayersState;

pub struct MainState {
    pub canvas: Option<CanvasState>,
    pub toolbar: ToolbarState,
    pub layers: Option<LayersState>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            canvas: None,
            toolbar: ToolbarState::new(),
            layers: None,
        }
    }
}
