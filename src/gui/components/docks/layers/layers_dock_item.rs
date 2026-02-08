use freya::prelude::*;

pub struct LayersDockItem {
    pub name: String,
    pub opacity: f32,
}

impl LayersDockItem {
    pub fn new(name: String) -> Self {
        LayersDockItem { name, opacity: 1.0 }
    }
}

impl From<LayersDockItem> for Element {
    fn from(value: LayersDockItem) -> Self {
        rect()
            .padding((4., 4.))
            .width(Size::fill())
            .height(Size::auto())
            .child(value.name)
            .border(Some(Border::new().width(1.)))
            .into()
    }
}
