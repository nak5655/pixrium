use crate::core::math::Radian;
use glam::{vec3, Vec2, Vec3};
use std::f32::consts::PI;

#[derive(PartialEq, Copy, Clone)]
pub struct ViewportState {
    pub look_at: Vec3,
    pub right: Vec3,
    pub fov: Radian,
    pub bounds: Vec2,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            look_at: vec3(1.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 1.0),
            fov: Radian(PI * 0.25),
            bounds: Vec2::default(),
        }
    }
}
