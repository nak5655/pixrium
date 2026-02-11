use freya::prelude::*;

pub mod color;
pub mod layers;

#[derive(Clone, PartialEq)]
pub struct Dock {
    pub title: String,
    pub children: Vec<Element>,
    key: DiffKey,
}

impl KeyExt for Dock {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Dock {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Dock {
    pub fn new() -> Self {
        Self {
            title: "".into(),
            children: vec![],
            key: DiffKey::None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }
}

impl Component for Dock {
    fn render(&self) -> impl IntoElement {
        rect()
            .direction(Direction::Vertical)
            .width(Size::fill())
            .padding(2.)
            .child(
                rect()
                    .width(Size::fill())
                    .padding((2., 4.))
                    .background((224, 224, 224))
                    .child(self.title.clone()),
            )
            .children(self.children.clone())
    }
}
