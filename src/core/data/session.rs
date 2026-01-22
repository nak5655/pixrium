use crate::core::data::project::Project;
use crate::core::data::states::ViewportState;
use crate::core::data::Layer;
use crate::core::math::Radian;
use glam::{Vec2, Vec3};
use skia_safe::images::raster_from_bitmap;
use skia_safe::Image;

pub struct Session {
    pub project: Project,
    pub selected_layer_index: usize,
    pub viewport_state: ViewportState,
    preview: Option<Image>,
}

impl Session {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            selected_layer_index: 0,
            viewport_state: ViewportState::new(),
            preview: None,
        }
    }

    pub fn layers(&self) -> Vec<&Layer> {
        self.project.layers.iter().collect()
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        if self.selected_layer_index < self.project.layers.len() {
            Some(&self.project.layers[self.selected_layer_index])
        } else {
            None
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut Layer> {
        if self.selected_layer_index < self.project.layers.len() {
            Some(&mut self.project.layers[self.selected_layer_index])
        } else {
            None
        }
    }

    pub fn update_preview(&mut self) {
        self.preview = self
            .selected_layer()
            .map(|layer| {
                return raster_from_bitmap(&layer.bitmap);
            })
            .flatten();
    }

    pub fn preview(&self) -> Option<Image> {
        self.preview.clone()
    }

    pub fn look_at(&self) -> Vec3 {
        self.viewport_state.look_at.clone()
    }

    pub fn right(&self) -> Vec3 {
        self.viewport_state.right.clone()
    }

    pub fn up(&self) -> Vec3 {
        self.viewport_state.right.cross(self.look_at()).normalize()
    }

    pub fn viewport_bounds(&self) -> Vec2 {
        self.viewport_state.bounds.clone()
    }

    pub fn fov(&self) -> f32 {
        self.viewport_state.fov.0.clone()
    }

    pub fn pan(&mut self, look_at: Vec3, right: Vec3) {
        self.viewport_state.look_at = look_at;
        self.viewport_state.right = right;
    }

    pub fn zoom(&mut self, fov: Radian) {
        self.viewport_state.fov = fov
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        // TODO: check pixels
        self.viewport_state == other.viewport_state
    }
}
