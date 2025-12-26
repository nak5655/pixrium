use freya::prelude::*;
use std::ops::Deref;

#[component]
pub fn CanvasInput(children: Element) -> Element {
    let mut dragging = use_signal(|| false);
    let mut position = use_signal(|| (0.0, 0.0));

    rsx! {
        rect {
            width: "fill",
            height: "fill",
            onmousedown: move |_| {
                dragging.set(true);
            },
            onmouseup: move |_| {
                dragging.set(false);
            },
            onmousemove: move |event: MouseEvent| {
                if *dragging.read() {
                    let x = event.get_element_coordinates().x;
                    let y = event.get_element_coordinates().y;
                    position.set((x, y));
                }
            },
            { children }
        }
    }
}
