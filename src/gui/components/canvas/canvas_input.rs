use freya::prelude::*;
use std::ops::Deref;
use skia_safe::Point;
use crate::core::inputs::Input;
use crate::core::logics::Console;
use crate::FreyaServices;
use crate::core::inputs::pointer_input::*;

#[component]
pub fn CanvasInput(console: Signal<Console<FreyaServices>>, children: Element) -> Element {
    rsx! {
        rect {
            width: "fill",
            height: "fill",
            onmousedown: move |event| {
                match get_button(&event) {
                    Some(button) => {
                        console.write().input(&Input::PointerInput(PointerInput::Down {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
            },
            onmousemove: move |event| {
                match get_button(&event) {
                    Some(button) => {
                        console.write().input(&Input::PointerInput(PointerInput::Move {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
            },
            onmouseup: move |event| {
                match get_button(&event) {
                    Some(button) => {
                        console.write().input(&Input::PointerInput(PointerInput::Up {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
            },
            { children }
        }
    }
}

fn get_button(event: &Event<MouseData>) -> Option<PointerButton> {
    match event.trigger_button {
        Some(MouseButton::Left) => Some(PointerButton::Left),
        Some(MouseButton::Right) => Some(PointerButton::Right),
        Some(MouseButton::Middle) => Some(PointerButton::Middle),
        _ => None,
    }
}

fn get_viewport_position(event: &Event<MouseData>) -> Point {
    let coords = event.element_coordinates;
    Point::new(coords.x as f32, coords.y as f32)
}

fn get_uv_position(event: &Event<MouseData>) -> Point {
    // TODO
    let coords = event.element_coordinates;
    Point::new(coords.x as f32, coords.y as f32)
}