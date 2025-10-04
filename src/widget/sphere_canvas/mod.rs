use std::sync::{Arc, RwLock};

use glam::{Vec2, Vec3, vec2, vec3};
use iced::advanced::graphics::core::event;
use iced::mouse::Button;
use iced::widget::shader;
use iced::widget::shader::wgpu;
use iced::{Rectangle, mouse};
use image::{EncodableLayout, GenericImageView};

pub fn sphere_canvas<'a, Message>(
    image: Arc<image::DynamicImage>,
    state: Arc<RwLock<SphereCanvasState>>,
) -> SphereCanvas<'a, Message> {
    SphereCanvas::new(image, state)
}

#[derive(Debug, Clone)]
pub enum SphereCanvasMessage {
    MousePressed {
        button: Button,
        position: Option<Vec2>,
    },
    MouseReleased {
        button: Button,
        position: Option<Vec2>,
    },
    MouseMoved {
        position: Vec2,
    },
    MouseWheel {
        delta: f32,
    },
    BoundsChanged(Rectangle),
}

pub struct SphereCanvas<'a, Message> {
    image: Arc<image::DynamicImage>,
    state: Arc<RwLock<SphereCanvasState>>,
    on_event: Option<Box<dyn Fn(SphereCanvasMessage) -> Message + 'a>>,
}

impl<'a, Message> SphereCanvas<'a, Message> {
    pub fn new(image: Arc<image::DynamicImage>, state: Arc<RwLock<SphereCanvasState>>) -> Self {
        SphereCanvas {
            image: image,
            state: state,
            on_event: None,
        }
    }

    pub fn on_event(mut self, f: impl Fn(SphereCanvasMessage) -> Message + 'static) -> Self {
        self.on_event = Some(Box::new(f));
        self
    }
}

impl<'a, Message> shader::Program<Message> for SphereCanvas<'a, Message> {
    type State = SphereCanvasState;
    type Primitive = SphereCanvasPrimitive;

    fn draw(
        &self,
        state: &Self::State,
        _cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        SphereCanvasPrimitive::new(
            bounds,
            SphereCanvasUniforms {
                aov: state.aov,
                look_at: state.look_at,
                up: state.up,
                right: state.right,
                ..Default::default()
            },
            self.image.clone(),
        )
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::widget::shader::Event,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
        shell: &mut iced::advanced::Shell<Message>,
    ) -> (
        iced::advanced::graphics::core::event::Status,
        Option<Message>,
    ) {
        if let Ok(read_state) = self.state.try_read() {
            read_state.clone_into(state);
        }

        if state.viewport_bounds != bounds {
            if let Some(f) = self.on_event.as_ref() {
                shell.publish(f(SphereCanvasMessage::BoundsChanged(bounds)))
            };
        }

        match event {
            shader::Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                if cursor.is_over(bounds) {
                    if let Some(f) = self.on_event.as_ref() {
                        shell.publish(f(SphereCanvasMessage::MousePressed {
                            button,
                            position: cursor.position().map(|p| vec2(p.x, p.y)),
                        }))
                    };
                }
            }
            shader::Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                if let Some(f) = self.on_event.as_ref() {
                    shell.publish(f(SphereCanvasMessage::MouseReleased {
                        button,
                        position: cursor.position().map(|p| vec2(p.x, p.y)),
                    }))
                };
            }
            shader::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(f) = self.on_event.as_ref() {
                    shell.publish(f(SphereCanvasMessage::MouseMoved {
                        position: vec2(position.x, position.y),
                    }))
                };
            }
            shader::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let delta_y = match delta {
                    mouse::ScrollDelta::Lines { x: _, y } => y,
                    mouse::ScrollDelta::Pixels { x: _, y } => y / 5.0,
                };
                if let Some(f) = self.on_event.as_ref() {
                    shell.publish(f(SphereCanvasMessage::MouseWheel { delta: delta_y }))
                };
            }
            _ => (),
        };

        if cursor.is_over(bounds) == false && state.mouse_button.is_none() {
            return (event::Status::Ignored, None);
        }

        (event::Status::Ignored, None)
    }
}

#[derive(Debug, Clone)]
pub struct SphereCanvasState {
    pub mouse_button: Option<Button>,
    pub mouse_point: Vec2,
    pub mouse_point_prev: Vec2,
    pub mouse_delta: Vec2,
    pub mouse_wheel_delta: f32,
    pub viewport_bounds: Rectangle,
    pub aov: f32,
    pub look_at: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}

impl Default for SphereCanvasState {
    fn default() -> Self {
        Self {
            mouse_button: None,
            mouse_point: vec2(0., 0.),
            mouse_point_prev: vec2(0., 0.),
            mouse_delta: vec2(0., 0.),
            mouse_wheel_delta: 0.0,
            viewport_bounds: Rectangle::default(),
            aov: 1.0,
            look_at: vec3(1., 0., 0.),
            up: vec3(0., 1., 0.),
            right: vec3(0., 0., 1.),
        }
    }
}

pub struct SphereCanvasPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    image: Arc<image::DynamicImage>,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl SphereCanvasPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        image: Arc<image::DynamicImage>,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sphere Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Sphere Texture"),
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sphere Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/sphere.wgsl").into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sphere Uniform Buffer"),
            size: std::mem::size_of::<SphereCanvasUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Sphere BindGroup Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: { wgpu::TextureSampleType::Float { filterable: true } },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_quad uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sphere Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sphere Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            image,
            texture,
            texture_view,
            sampler,
        }
    }

    fn update(&mut self, queue: &wgpu::Queue, uniforms: &SphereCanvasUniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}

/// A struct that represents a uniform for the shader.
/// Its members have to be aligned to 16bytes.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct SphereCanvasUniforms {
    aov: f32,
    _padding1: [f32; 3],
    look_at: glam::Vec3,
    _padding2: [f32; 1],
    up: glam::Vec3,
    _padding3: [f32; 1],
    right: glam::Vec3,
    _padding4: [f32; 1],
}

impl Default for SphereCanvasUniforms {
    fn default() -> Self {
        Self {
            aov: 1.0,
            look_at: glam::vec3(1.0, 0.0, 0.0),
            up: glam::vec3(0.0, 1.0, 0.0),
            right: glam::vec3(0.0, 0.0, 1.0),

            _padding1: [0.0; 3],
            _padding2: [0.0; 1],
            _padding3: [0.0; 1],
            _padding4: [0.0; 1],
        }
    }
}

#[derive(Debug)]
pub struct SphereCanvasPrimitive {
    uniforms: SphereCanvasUniforms,
    image: Arc<image::DynamicImage>,
}

impl SphereCanvasPrimitive {
    pub fn new(
        bounds: Rectangle,
        unifroms: SphereCanvasUniforms,
        image: Arc<image::DynamicImage>,
    ) -> Self {
        Self {
            uniforms: unifroms,
            image: image,
        }
    }
}

impl shader::Primitive for SphereCanvasPrimitive {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        _bounds: &Rectangle,
        viewport: &shader::Viewport,
    ) {
        let mut is_texture_invalidated = false;

        if storage.has::<SphereCanvasPipeline>() {
            let pipeline = storage.get_mut::<SphereCanvasPipeline>().unwrap();
            if pipeline.image != self.image {
                let new_pipeline = SphereCanvasPipeline::new(device, format, self.image.clone());
                is_texture_invalidated = true;
                storage.store(new_pipeline)
            }
        } else {
            let pipeline = SphereCanvasPipeline::new(device, format, self.image.clone());
            is_texture_invalidated = true;
            storage.store(pipeline);
        }

        let pipeline = storage.get_mut::<SphereCanvasPipeline>().unwrap();

        if is_texture_invalidated {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &pipeline.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                pipeline.image.to_rgba8().as_bytes(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * self.image.dimensions().0),
                    rows_per_image: Some(self.image.dimensions().1),
                },
                pipeline.texture.size(),
            );
        }

        pipeline.update(queue, &self.uniforms);
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<SphereCanvasPipeline>().unwrap();
        pipeline.render(target, encoder, *clip_bounds);
    }
}
