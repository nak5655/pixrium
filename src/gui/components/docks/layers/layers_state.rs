use crate::gui::components::docks::layers::LayersDockItem;

pub struct LayersState {
    pub layers: Vec<LayersDockItem>,
    pub selected_layer: Option<LayersDockItem>,
}

impl LayersState {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            selected_layer: None,
        }
    }
}
