#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::core::inputs::{Input, KeyboardInput};
use crate::core::inputs::{PointerButton, PointerInput};
use crate::core::logics::Console;
use crate::gui::components::canvas::canvas_view::CanvasView;
use crate::FreyaServices;
use freya::prelude::*;
use glam::{vec2, Vec2};

pub fn canvas(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    let mut least_viewport_position = use_state(|| Vec2::default());
    let mut pressed_button = use_state(|| None);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .maybe_child(
            console
                .read()
                .session
                .as_ref()
                .map(|session| CanvasView::new(session).expanded()),
        )
        .on_mouse_down(move |event| match get_button(&event) {
            Some(button) => {
                pressed_button.set(Some(button));
                console.write().input(&Input::Pointer(PointerInput::Down {
                    button,
                    viewport_position: get_viewport_position(&event),
                }))
            }
            _ => {}
        })
        .on_mouse_move(move |event| {
            match *pressed_button.peek() {
                Some(button) => console.write().input(&Input::Pointer(PointerInput::Move {
                    button: Some(button),
                    viewport_position: get_viewport_position(&event),
                })),
                _ => {}
            }
            least_viewport_position.set(get_viewport_position(&event));
        })
        .on_mouse_up(move |event| {
            match *pressed_button.peek() {
                Some(button) => console.write().input(&Input::Pointer(PointerInput::Up {
                    button,
                    viewport_position: get_viewport_position(&event),
                })),
                _ => {}
            }
            pressed_button.set(None);
        })
        .on_wheel(move |event: Event<WheelEventData>| {
            console.write().input(&Input::Pointer(PointerInput::Scroll {
                delta: vec2(event.delta_x as f32, event.delta_y as f32),
                viewport_position: least_viewport_position.peek().clone(),
            }))
        })
        .on_global_key_down(move |event| {
            console.write().input(&Input::Keyboard(KeyboardInput::Down {
                key: get_key(&event),
            }))
        })
        .on_sized(move |event: Event<SizedEventData>| {
            console.with_mut(|mut s| {
                if let Some(session) = s.session.as_mut() {
                    session.viewport_state.bounds.x = event.area.width();
                    session.viewport_state.bounds.y = event.area.height();
                }
            });
        })
}

fn get_button(event: &Event<MouseEventData>) -> Option<PointerButton> {
    match event.button {
        Some(MouseButton::Left) => Some(PointerButton::Left),
        Some(MouseButton::Right) => Some(PointerButton::Right),
        Some(MouseButton::Middle) => Some(PointerButton::Middle),
        Some(_) => Some(PointerButton::Left),
        _ => None,
    }
}

fn get_viewport_position(event: &Event<MouseEventData>) -> Vec2 {
    let coords = event.element_location;
    Vec2::new(coords.x as f32, coords.y as f32)
}

fn get_key(event: &Event<KeyboardEventData>) -> crate::core::inputs::Key {
    match &event.key {
        Key::Character(s) => crate::core::inputs::Key::Character(s.clone()),
        _ => crate::core::inputs::Key::Unidentified,
    }
}
