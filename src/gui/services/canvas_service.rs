use crate::core::math::Radian;
use crate::core::services::CanvasService;
use crate::gui::components::canvas::CanvasState;
use freya::prelude::*;
use glam::{Vec2, Vec3};

pub struct CanvasServiceImpl {
    pub canvas_state: State<CanvasState>,
}

impl CanvasService for CanvasServiceImpl {
    fn look_at(&self) -> Vec3 {
        self.canvas_state.peek().look_at
    }

    fn right(&self) -> Vec3 {
        self.canvas_state.peek().right
    }

    fn up(&self) -> Vec3 {
        self.right().cross(self.look_at()).normalize()
    }

    fn viewport_bounds(&self) -> Vec2 {
        self.canvas_state.peek().viewport_bounds
    }

    fn fov(&self) -> f32 {
        self.canvas_state.peek().fov.0
    }

    fn pan(&mut self, look_at: Vec3, right: Vec3) {
        self.canvas_state.with_mut(|mut s| {
            s.look_at = look_at;
            s.right = right;
        });
    }

    fn zoom(&mut self, fov: Radian) {
        self.canvas_state.write().fov = fov
    }
}
