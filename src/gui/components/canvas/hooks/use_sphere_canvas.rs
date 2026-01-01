use std::sync::Arc;
use freya::hooks::{use_canvas, use_node_signal, use_platform, UniformValue, UniformsBuilder, UseCanvas};
use freya::prelude::*;
use skia_safe::{Color, Data, Image, Paint, Rect, RuntimeEffect, SamplingOptions, TileMode};
use crate::gui::components::canvas::CanvasState;

pub fn use_sphere_canvas(canvas_state: Signal<CanvasState>) -> UseCanvas {
    use_canvas(move || {
        //
        let sksl_frag = include_str!("./canvas.sksl");
        let runtime_effect = RuntimeEffect::make_for_shader(sksl_frag, None).unwrap();
        let shader_wrapper = Arc::new(ShaderWrapper(runtime_effect));

        let state = canvas_state.read();
        let look_at = state.look_at;
        let fov = state.fov.0;
        let right = state.right;
        // 視点ベクトルから見て右ベクトルと直交
        let up = right.cross(look_at).normalize();

        let viewport_size = state.viewport_bounds;

        move |ctx| {
            //
            let image = Image::from_encoded(Data::new_copy(include_bytes!("./sample.jpg"))).unwrap();
            let texture = image.to_shader(Some((TileMode::Repeat, TileMode::Repeat)), SamplingOptions::default(), None).expect("failed to load texture");

            let mut builder = UniformsBuilder::default();
            builder.set(
                "uLookAt",
                UniformValue::FloatVec(vec![look_at.x, look_at.y, look_at.z]),
            );
            builder.set(
                "uUp",
                UniformValue::FloatVec(vec![up.x, up.y, up.z]),
            );            builder.set(
                "uRight",
                UniformValue::FloatVec(vec![right.x, right.y, right.z]),
            );
            builder.set(
                "uAov",
                UniformValue::Float(fov),
            );
            builder.set(
                "uViewportSize",
                UniformValue::FloatVec(vec![viewport_size.x, viewport_size.y]),
            );
            builder.set(
                "uTexSize",
                UniformValue::FloatVec(vec![image.width() as f32, image.height() as f32]),
            );

            let uniforms = Data::new_copy(&builder.build(&shader_wrapper.0));

            let shader = shader_wrapper.0.make_shader(uniforms, &[texture.into()], None).unwrap();

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(Color::WHITE);
            paint.set_shader(shader);

            ctx.canvas.draw_rect(
                Rect::new(
                    ctx.area.min_x(),
                    ctx.area.min_y(),
                    ctx.area.max_x(),
                    ctx.area.max_y(),
                ),
                &paint,
            );
        }
    })
}

struct ShaderWrapper(RuntimeEffect);

unsafe impl Sync for ShaderWrapper {}
unsafe impl Send for ShaderWrapper {}
