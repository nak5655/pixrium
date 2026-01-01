use std::f32::consts::PI;
use glam::{vec3, Vec2, Vec3};
use crate::core::math::{LatLon, Radian};
use crate::latlon;

#[derive(PartialEq, Copy, Clone)]
pub struct CanvasState {
    pub look_at: Vec3,
    pub right: Vec3,
    pub fov: Radian,
    pub viewport_bounds: Vec2,
}

impl CanvasState {
    pub fn new() -> Self {
        Self {
            look_at: vec3(1.0, 0.0, 0.0),
            right: vec3(0.0, 1.0, 0.0),
            fov: Radian(PI * 0.25),
            viewport_bounds: Vec2::default()
        }
    }
}