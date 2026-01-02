use freya::prelude::*;

#[component]
pub fn LayerDockItem(name: String) -> Element {
    rsx! {
        rect {
            width: "fill",
            height: "auto",
            label {
                { name }
            }
        }
    }
}
