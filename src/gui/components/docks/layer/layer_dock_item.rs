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
            .padding((4., 4.))
            .width(Size::fill())
            .height(Size::auto())
            .child(value.name)
            .border(Some(Border::new().width(1.)))
            .into()
    }
}
