use glam::{vec3, Vec3};
use skia_safe::Size;
use std::f32::consts::PI;

#[derive(PartialEq, Copy, Clone)]
pub struct ViewportState {
    pub look_at: Vec3,
    pub right: Vec3,
    pub fov: f32,
    pub size: Size,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            look_at: vec3(1.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 1.0),
            fov: PI * 0.25,
            size: Size::default(),
        }
    }

    pub fn up(&self) -> Vec3 {
        self.right.cross(self.look_at).normalize()
    }
}
