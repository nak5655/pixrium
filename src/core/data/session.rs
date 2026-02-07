use crate::core::data::project::Project;
use crate::core::data::{Layer, SessionState};
use crate::core::output::OutputSignal;
use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{
    images, surfaces, AlphaType, ColorSpace, ColorType, ISize, ImageInfo, Paint, Rect, Surface,
};
use std::sync::mpsc::Sender;

pub struct Session {
    output: Sender<OutputSignal>,
    pub project: Project,
    pub state: SessionState,
    preview: Surface,
}

impl Session {
    pub fn new(project: Project, output: Sender<OutputSignal>) -> Self {
        let preview_image_info = ImageInfo::new(
            ISize::new(project.width as i32, project.height as i32),
            ColorType::RGBAF16,
            AlphaType::Opaque,
            Some(ColorSpace::new_srgb()),
        );

        let surface = surfaces::raster(&preview_image_info, None, None).unwrap();
        let state = SessionState::new();

        output.send(OutputSignal::Viewport(state.viewport)).unwrap();

        Self {
            output,
            project,
            state,
            preview: surface,
        }
    }

    pub fn layers(&self) -> Vec<&Layer> {
        self.project.layers.iter().collect()
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        if self.state.selected_layer_index < self.project.layers.len() {
            Some(&self.project.layers[self.state.selected_layer_index])
        } else {
            None
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut Layer> {
        if self.state.selected_layer_index < self.project.layers.len() {
            Some(&mut self.project.layers[self.state.selected_layer_index])
        } else {
            None
        }
    }

    pub fn update_frame(&mut self, bounds: Rect) {
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

        let snapshot = self.preview.image_snapshot();

        _ = self.output.send(OutputSignal::Frame(snapshot));
        _ = self
            .output
            .send(OutputSignal::Viewport(self.state.viewport));
    }

    pub fn state_mut(&mut self) -> &mut SessionState {
        &mut self.state
    }

    pub fn update_viewport(&mut self) {
        _ = self
            .output
            .send(OutputSignal::Viewport(self.state.viewport));
    }
}
