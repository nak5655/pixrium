#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::core::input::{InputSignal, KeyboardInput};
use crate::core::input::{PointerButton, PointerInput};
use crate::gui::components::canvas::CanvasShader;
use crate::gui::windows::main::MainState;
use crate::{FreyaServices, OutputChannel};
use freya::prelude::*;
use freya_radio::prelude::use_radio;
use glam::{vec2, Vec2};
use skia_safe::{ISize, RuntimeEffect};
use std::sync::mpsc::Sender;

pub fn canvas_view(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let canvas_radio = use_radio::<MainState, OutputChannel>(OutputChannel::Canvas);

    use_hook(|| {
        let mut ticker = consume_root_context::<RenderingTicker>();
        let platform = Platform::get();

        spawn(async move {
            loop {
                ticker.tick().await;
                platform.send(UserEvent::RequestRedraw);
            }
        });
    });

    let mut least_viewport_position = use_state(|| Vec2::default());
    let mut pressed_button = use_state(|| None);

    let runtime_effect = use_hook(|| {
        let sksl_frag = include_str!("./canvas.sksl");
        RuntimeEffect::make_for_shader(sksl_frag, None).unwrap()
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .maybe_child(
            canvas_radio.read().canvas.as_ref().map(|canvas_state| {
                CanvasShader::new(runtime_effect, canvas_state.clone()).expanded()
            }),
        )
        .on_mouse_down(move |event| match get_button(&event) {
            Some(button) => {
                pressed_button.set(Some(button));
                _ = input.write().send(InputSignal::Pointer(PointerInput::Down {
                    button,
                    viewport_position: get_viewport_position(&event),
                }));
            }
            _ => {}
        })
        .on_mouse_move(move |event| {
            match *pressed_button.peek() {
                Some(button) => {
                    _ = input.write().send(InputSignal::Pointer(PointerInput::Move {
                        button: Some(button),
                        viewport_position: get_viewport_position(&event),
                    }))
                }
                _ => {}
            }
            least_viewport_position.set(get_viewport_position(&event));
        })
        .on_mouse_up(move |event| {
            match *pressed_button.peek() {
                Some(button) => {
                    _ = input.write().send(InputSignal::Pointer(PointerInput::Up {
                        button,
                        viewport_position: get_viewport_position(&event),
                    }))
                }
                _ => {}
            }
            pressed_button.set(None);
        })
        .on_wheel(move |event: Event<WheelEventData>| {
            _ = input
                .write()
                .send(InputSignal::Pointer(PointerInput::Scroll {
                    delta: vec2(event.delta_x as f32, event.delta_y as f32),
                    viewport_position: least_viewport_position.peek().clone(),
                }))
        })
        .on_global_key_down(move |event| {
            _ = input
                .write()
                .send(InputSignal::Keyboard(KeyboardInput::Down {
                    key: get_key(&event),
                }))
        })
        .on_sized(move |event: Event<SizedEventData>| {
            _ = input.write().send(InputSignal::ViewportResized(ISize::new(
                event.area.width() as i32,
                event.area.height() as i32,
            )))
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

fn get_key(event: &Event<KeyboardEventData>) -> crate::core::input::Key {
    match &event.key {
        Key::Character(s) => crate::core::input::Key::Character(s.clone()),
        _ => crate::core::input::Key::Unidentified,
    }
}
