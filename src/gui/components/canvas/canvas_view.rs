#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use skia_safe::{Color, Data, Image, Paint, Point, Rect, RuntimeEffect, SamplingOptions, TileMode};
use std::{
    sync::Arc,
    time::Instant,
};
use crate::core::inputs::Input;
use crate::core::inputs::pointer_input::{PointerButton, PointerInput};
use crate::core::logics::Console;
use crate::FreyaServices;
use crate::gui::components::canvas::CanvasState;
use crate::gui::components::canvas::hooks::use_sphere_canvas;

#[component]
pub fn CanvasView(console: Signal<Console<FreyaServices>>, canvas_state: Signal<CanvasState>) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    let canvas = use_sphere_canvas(canvas_state);

    let mut pressed_button = use_signal(|| None);

    use_effect(move || {
        let area = size.peek().area;
        canvas_state.with_mut(|s| {
            s.viewport_bounds.x = area.width();
            s.viewport_bounds.y = area.height();
        });
    });

    rsx! {
        rect {
            canvas_reference: canvas.attribute(),
            reference,
            background: "black",
            width: "fill",
            height: "fill",
            onmousedown: move |event| {
                match get_button(&event) {
                    Some(button) => {
                        pressed_button.set(Some(button));
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
                match *pressed_button.peek() {
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
                match *pressed_button.peek() {
                    Some(button) => {
                        console.write().input(&Input::PointerInput(PointerInput::Up {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
                pressed_button.set(None);
            },

        }
    }
}

fn get_button(event: &Event<MouseData>) -> Option<PointerButton> {
    match event.trigger_button {
        Some(MouseButton::Left) => Some(PointerButton::Left),
        Some(MouseButton::Right) => Some(PointerButton::Right),
        Some(MouseButton::Middle) => Some(PointerButton::Middle),
        Some(_) => Some(PointerButton::Left),
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