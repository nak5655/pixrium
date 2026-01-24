use crate::core::data::project::Project;
use crate::core::data::states::ViewportState;
use crate::core::data::Layer;
use crate::core::math::Radian;
use glam::{Vec2, Vec3};
use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{
    images, surfaces, AlphaType, ColorSpace, ColorType, ISize, Image, ImageInfo, Paint, Rect,
    Surface,
};

pub struct Session {
    pub project: Project,
    pub selected_layer_index: usize,
    pub viewport_state: ViewportState,
    preview: Surface,
    snapshot: Image,
}

impl Session {
    pub fn new(project: Project) -> Self {
        let preview_image_info = ImageInfo::new(
            ISize::new(project.width as i32, project.height as i32),
            ColorType::RGBAF16,
            AlphaType::Opaque,
            Some(ColorSpace::new_srgb()),
        );

        let mut surface = surfaces::raster(&preview_image_info, None, None).unwrap();
        let snapshot = surface.image_snapshot();

        Self {
            project,
            selected_layer_index: 0,
            viewport_state: ViewportState::new(),
            preview: surface,
            snapshot,
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

    pub fn update_preview(&mut self, bounds: Rect) {
        let canvas = self.preview.canvas();
        let paint = Paint::default();

        for layer in self.project.layers.iter().rev() {
            let layer_image = images::raster_from_bitmap(&layer.bitmap).unwrap();
            canvas.draw_image_rect(
                layer_image,
                Some((&bounds, SrcRectConstraint::Fast)),
                bounds,
                &paint,
            );
        }

        self.snapshot = self.preview.image_snapshot();
    }

    pub fn preview(&self) -> Image {
        self.snapshot.clone()
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
