use crate::core::data::ViewportState;
use freya::prelude::*;
use freya_core::data::LayoutData;
use freya_core::diff_key::DiffKey;
use freya_core::element::{Element, ElementExt, RenderContext};
use freya_core::integration::{ContainerExt, DiffModifies, LayoutExt};
use skia_safe::{Color, Data, Image, Paint, Rect, RuntimeEffect, SamplingOptions, TileMode};
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

pub struct CanvasShader {
    layout_data: LayoutData,
    runtime_effect: RuntimeEffect,
    viewport_state: ViewportState,
    image: Image,
}

impl CanvasShader {
    pub fn new(runtime_effect: RuntimeEffect, image: Image, viewport_state: ViewportState) -> Self {
        Self {
            layout_data: LayoutData::default(),
            runtime_effect,
            image,
            viewport_state,
        }
    }
}

impl ElementExt for CanvasShader {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(shader) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return true;
        };

        if (self.image.unique_id() != shader.image.unique_id()) {
            return true;
        }

        let is_equal = self.viewport_state == shader.viewport_state;

        !is_equal
    }

    fn diff(&self, other: &std::rc::Rc<dyn ElementExt>) -> DiffModifies {
        let Some(element) = (other.as_ref() as &dyn Any).downcast_ref::<CanvasShader>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if (self.image.unique_id() != element.image.unique_id()) {
            diff.insert(DiffModifies::INNER_LAYOUT);
        }

        let is_equal = self.viewport_state == element.viewport_state;
        if !is_equal {
            diff.insert(DiffModifies::INNER_LAYOUT);
        }

        if self.layout_data != element.layout_data {
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn layout(&'_ self) -> std::borrow::Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout_data)
    }

    fn render(&self, context: RenderContext) {
        let texture = self
            .image
            .to_shader(
                Some((TileMode::Repeat, TileMode::Repeat)),
                SamplingOptions::default(),
                None,
            )
            .expect("failed to load texture");

        let canvas_state = self.viewport_state;
        let look_at = canvas_state.look_at;
        let fov = canvas_state.fov.0;
        let right = canvas_state.right;
        // 視点ベクトルから見て右ベクトルと直交
        let up = right.cross(look_at).normalize();

        let mut builder = UniformsBuilder::default();
        builder.set(
            "u_look_at",
            UniformValue::FloatVec(vec![look_at.x, look_at.y, look_at.z]),
        );
        builder.set("u_up", UniformValue::FloatVec(vec![up.x, up.y, up.z]));
        builder.set(
            "u_right",
            UniformValue::FloatVec(vec![right.x, right.y, right.z]),
        );
        builder.set("u_aov", UniformValue::Float(fov));
        builder.set(
            "u_viewport_position",
            UniformValue::FloatVec(vec![
                context.layout_node.area.min_x(),
                context.layout_node.area.min_y(),
            ]),
        );
        builder.set(
            "u_viewport_size",
            UniformValue::FloatVec(vec![
                context.layout_node.area.width(),
                context.layout_node.area.height(),
            ]),
        );
        builder.set(
            "u_texture_size",
            UniformValue::FloatVec(vec![self.image.width() as f32, self.image.height() as f32]),
        );

        let uniforms = Data::new_copy(&builder.build(&self.runtime_effect));

        let shader = self
            .runtime_effect
            .make_shader(uniforms, &[texture.into()], None)
            .expect("failed to make shader (set uniforms)");

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::WHITE);
        paint.set_shader(shader);

        context.canvas.draw_rect(
            Rect::new(
                context.layout_node.area.min_x(),
                context.layout_node.area.min_y(),
                context.layout_node.area.max_x(),
                context.layout_node.area.max_y(),
            ),
            &paint,
        );
    }
}

impl LayoutExt for CanvasShader {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout_data
    }
}
impl ContainerExt for CanvasShader {}

impl From<CanvasShader> for Element {
    fn from(value: CanvasShader) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value),
            elements: Vec::new(),
        }
    }
}

/// Pass uniform values to a Shader.
#[derive(Default)]
struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>,
}

/// Uniform value to be passed to a Shader.
enum UniformValue {
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
