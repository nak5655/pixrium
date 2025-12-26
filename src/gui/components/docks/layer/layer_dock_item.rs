use freya::prelude::*;

#[component]
pub fn LayerDockItem(name: String) -> Element {
    rsx!(
        label {
            { name }
        }
    )
}
