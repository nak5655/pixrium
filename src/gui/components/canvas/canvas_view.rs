#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use skia_safe::{Color, Data, Image, Paint, Rect, RuntimeEffect, SamplingOptions, TileMode};
use std::{
    sync::Arc,
    time::Instant,
};

#[component]
pub fn CanvasView() -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    let canvas = use_canvas(|| {
        //
        let sksl_frag = include_str!("./canvas.sksl");

        let runtime_effect = RuntimeEffect::make_for_shader(sksl_frag, None).unwrap();

        let shader_wrapper = Arc::new(ShaderWrapper(runtime_effect));

        move |ctx| {
            //
            let image = Image::from_encoded(Data::new_copy(include_bytes!("./sample.jpg"))).unwrap();
            let texture = image.to_shader(Some((TileMode::Repeat, TileMode::Repeat)), SamplingOptions::default(), None).expect("failed to load texture");

            let mut builder = UniformsBuilder::default();
            builder.set(
                "uLookAt",
                UniformValue::FloatVec(vec![1.0, 0.0, 0.0]),
            );
            builder.set(
                "uUp",
                UniformValue::FloatVec(vec![0.0, 1.0, 0.0]),
            );            builder.set(
                "uRight",
                UniformValue::FloatVec(vec![0.0, 0.0, 1.0]),
            );
            builder.set(
                "uAov",
                UniformValue::Float(1.0),
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
    });

    rsx! {
        rect {
            canvas_reference: canvas.attribute(),
            reference,
            background: "black",
            width: "fill",
            height: "fill",
        }
    }
}

struct ShaderWrapper(RuntimeEffect);

unsafe impl Sync for ShaderWrapper {}
unsafe impl Send for ShaderWrapper {}
