use freya::prelude::*;
use freya_core::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToggleButton {
    children: Vec<Element>,
    on_select: Option<EventHandler<()>>,
    selected: bool,
    key: DiffKey,
}

impl KeyExt for ToggleButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for ToggleButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for ToggleButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl ToggleButton {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            on_select: None,
            selected: false,
            key: DiffKey::None,
        }
    }

    pub fn on_select(mut self, on_select: impl Into<EventHandler<()>>) -> Self {
        self.on_select = Some(on_select.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Component for ToggleButton {
    fn render(&self) -> impl IntoElement {
        let on_press = {
            let on_select = self.on_select.clone();
            move |e: Event<PressEventData>| {
                if let Some(on_select) = &on_select {
                    e.stop_propagation();
                    on_select.call(());
                }
            }
        };

        rect()
            .direction(Direction::Horizontal)
            .padding(8.)
            .spacing(8.)
            .cross_align(Alignment::center())
            .on_press(on_press)
            .background(if self.selected {
                Color::from_rgb(196, 225, 255)
            } else {
                Color::TRANSPARENT
            })
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
