use freya::prelude::*;

pub struct LayerDockItem {
    pub name: String,
}

impl LayerDockItem {
    pub fn new(name: String) -> Self {
        LayerDockItem { name }
    }
}

impl From<LayerDockItem> for Element {
    fn from(value: LayerDockItem) -> Self {
        rect()
            .width(Size::fill())
            .height(Size::auto())
            .child(value.name)
            .into()
    }
}
