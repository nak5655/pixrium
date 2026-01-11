use crate::core::data::project::Project;
use crate::core::data::states::ViewportState;
use crate::core::data::Layer;
use crate::core::math::Radian;
use glam::{Vec2, Vec3};
use skia_safe::images::raster_from_bitmap;
use skia_safe::Image;

pub struct Session {
    pub project: Option<Project>,
    pub selected_layer_index: usize,
    pub viewport_state: ViewportState,
    preview: Option<Image>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
            selected_layer_index: 0,
            viewport_state: ViewportState::new(),
            preview: None,
        }
    }

    pub fn layers(&self) -> Vec<&Layer> {
        match &self.project {
            Some(project) => project.layers.iter().collect(),
            None => return vec![],
        }
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        let project = match &self.project {
            Some(project) => project,
            None => return None,
        };

        if 0 <= self.selected_layer_index && self.selected_layer_index < project.layers.len() {
            Some(&project.layers[self.selected_layer_index])
        } else {
            None
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut Layer> {
        let mut project = match &mut self.project {
            Some(project) => project,
            None => return None,
        };

        if 0 <= self.selected_layer_index && self.selected_layer_index < project.layers.len() {
            Some(&mut project.layers[self.selected_layer_index])
        } else {
            None
        }
    }

    pub fn update_preview(&mut self) {
        self.preview = self
            .selected_layer()
            .map(|layer| raster_from_bitmap(&layer.bitmap))
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
