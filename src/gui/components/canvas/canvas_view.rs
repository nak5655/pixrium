#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::core::inputs::{Input, KeyboardInput};
use crate::core::inputs::{PointerButton, PointerInput};
use crate::core::logics::Console;
use crate::gui::components::canvas::CanvasShader;
use crate::FreyaServices;
use freya::prelude::*;
use glam::{vec2, Vec2};
use skia_safe::RuntimeEffect;
use std::collections::HashMap;

pub fn canvas_view(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
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
    let mut least_uv_position = use_state(|| Vec2::default());
    let mut pressed_button = use_state(|| None);

    let runtime_effect = use_hook(|| {
        let sksl_frag = include_str!("./canvas.sksl");
        RuntimeEffect::make_for_shader(sksl_frag, None).unwrap()
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .maybe_child(console.read().session.preview().map(|img| {
            CanvasShader::new(
                runtime_effect,
                img,
                console.peek().session.viewport_state.clone(),
            )
            .expanded()
        }))
        .on_mouse_down(move |event| match get_button(&event) {
            Some(button) => {
                pressed_button.set(Some(button));
                console.write().input(&Input::Pointer(PointerInput::Down {
                    button,
                    viewport_position: get_viewport_position(&event),
                    uv_position: get_uv_position(&event),
                }))
            }
            _ => {}
        })
        .on_mouse_move(move |event| {
            match *pressed_button.peek() {
                Some(button) => console.write().input(&Input::Pointer(PointerInput::Move {
                    button,
                    viewport_position_delta: get_viewport_position(&event)
                        - *least_viewport_position.read(),
                    viewport_position: get_viewport_position(&event),
                    uv_position_delta: get_uv_position(&event) - *least_uv_position.read(),
                    uv_position: get_uv_position(&event),
                })),
                _ => {}
            }
            least_viewport_position.set(get_viewport_position(&event));
            least_uv_position.set(get_uv_position(&event));
        })
        .on_mouse_up(move |event| {
            match *pressed_button.peek() {
                Some(button) => console.write().input(&Input::Pointer(PointerInput::Up {
                    button,
                    viewport_position: get_viewport_position(&event),
                    uv_position: get_uv_position(&event),
                })),
                _ => {}
            }
            pressed_button.set(None);
        })
        .on_wheel(move |event: Event<WheelEventData>| {
            console.write().input(&Input::Pointer(PointerInput::Scroll {
                delta: vec2(event.delta_x as f32, event.delta_y as f32),
                viewport_position: least_viewport_position.peek().clone(),
                uv_position: least_uv_position.peek().clone(),
            }))
        })
        .on_global_key_down(move |event| {
            console.write().input(&Input::Keyboard(KeyboardInput::Down {
                key: get_key(&event),
            }))
        })
        .on_sized(move |event: Event<SizedEventData>| {
            console.with_mut(|mut s| {
                s.session.viewport_state.bounds.x = event.area.width();
                s.session.viewport_state.bounds.y = event.area.height();
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

fn get_uv_position(event: &Event<MouseEventData>) -> Vec2 {
    // TODO
    let coords = event.element_location;
    Vec2::new(coords.x as f32, coords.y as f32)
}

fn get_key(event: &Event<KeyboardEventData>) -> crate::core::inputs::Key {
    match &event.key {
        Key::Character(s) => crate::core::inputs::Key::Character(s.clone()),
        _ => crate::core::inputs::Key::Unidentified,
    }
}

/// Pass uniform values to a Shader.
#[derive(Default)]
pub struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>,
}

/// Uniform value to be passed to a Shader.
pub enum UniformValue {
    Float(f32),
    #[allow(dead_code)]
    FloatVec(Vec<f32>),
}

impl UniformsBuilder {
    /// Set a uniform value.
    pub fn set(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    /// Build the uniform bytes.
    pub fn build(&self, shader: &RuntimeEffect) -> Vec<u8> {
        let mut values = Vec::new();

        for uniform in shader.uniforms().iter() {
            let value = self.uniforms.get(uniform.name()).unwrap();
            match &value {
                UniformValue::Float(f) => {
                    values.extend(f.to_le_bytes());
                }
                UniformValue::FloatVec(f) => {
                    for n in f {
                        values.extend(n.to_le_bytes());
                    }
                }
            }
        }

        values
    }
}
