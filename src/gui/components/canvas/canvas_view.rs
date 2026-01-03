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
use glam::{vec2, Vec2, Vec3};
use crate::core::inputs::{Input, KeyboardInput};
use crate::core::inputs::{PointerButton, PointerInput};
use crate::core::logics::Console;
use crate::core::math::Radian;
use crate::core::services::{CanvasService, Services};
use crate::FreyaServices;
use crate::gui::components::canvas::CanvasState;
use crate::gui::components::canvas::hooks::use_sphere_canvas;

#[component]
pub fn CanvasView(console: Signal<Console<FreyaServices>>, canvas_state: Signal<CanvasState>) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    platform.invalidate_drawing_area(size.peek().area);
    platform.request_animation_frame();

    let canvas = use_sphere_canvas(canvas_state);

    let mut least_viewport_position = use_signal(|| Vec2::default());
    let mut least_uv_position = use_signal(|| Vec2::default());
    let mut pressed_button = use_signal(|| None);

    use_memo(move || {
        let area = size.read().area;
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
                        console.write().input(&Input::Pointer(PointerInput::Down {
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
                        console.write().input(&Input::Pointer(PointerInput::Move {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
                least_viewport_position.set(get_viewport_position(&event));
                least_uv_position.set(get_uv_position(&event));
            },
            onmouseup: move |event| {
                match *pressed_button.peek() {
                    Some(button) => {
                        console.write().input(&Input::Pointer(PointerInput::Up {
                            button,
                            viewport_position: get_viewport_position(&event),
                            uv_position: get_uv_position(&event),
                        }))
                    },
                    _ => { }
                }
                pressed_button.set(None);
            },
            onwheel: move |event| {
                console.write().input(&Input::Pointer(PointerInput::Scroll {
                    delta: vec2(event.data.get_delta_x() as f32, event.data.get_delta_y() as f32),
                    viewport_position: least_viewport_position.peek().clone(),
                    uv_position: least_uv_position.peek().clone()
                }))
            },
            onglobalkeydown: move |event| {
                console.write().input(&Input::Keyboard(KeyboardInput::Down {
                    key: get_key(&event),
                }))
            }
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

fn get_viewport_position(event: &Event<MouseData>) -> Vec2 {
    let coords = event.element_coordinates;
    Vec2::new(coords.x as f32, coords.y as f32)
}

fn get_uv_position(event: &Event<MouseData>) -> Vec2 {
    // TODO
    let coords = event.element_coordinates;
    Vec2::new(coords.x as f32, coords.y as f32)
}

fn get_key(event: &Event<KeyboardData>) -> crate::core::inputs::Key {
    match &event.key {
        Key::Character(s) => crate::core::inputs::Key::Character(s.clone()),
        _ => crate::core::inputs::Key::Unidentified,
    }
}