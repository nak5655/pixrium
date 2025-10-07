use std::sync::{Arc, RwLock};

use glam::{Quat, Vec2};
use iced::advanced::graphics::core::event::Status;
use iced::mouse;
use image::{GenericImage, GenericImageView};

use crate::tool::Tool;
use crate::widget::sphere_canvas::SphereCanvasState;

#[derive(Debug)]
pub struct PenTool {
    pub name: String,
    pub icon: char,
}

impl PenTool {
    pub fn new() -> Self {
        Self {
            name: "Pen".to_string(),
            icon: '\u{eb04}',
        }
    }
}

impl Tool for PenTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn icon(&self) -> char {
        self.icon
    }

    fn on_mouse_moved(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        // Get the UV position before acquiring mutable borrow
        let uv_position = {
            if let Ok(canvas_state) = canvas_state.try_read() {
                canvas_state.get_uv_position()
            } else {
                None
            }
        };

        if let Ok(mut canvas_state) = canvas_state.try_write() {
            if canvas_state.mouse_button == Some(mouse::Button::Left) {
                if let Some(rw_image) = canvas_state.image_bytes.clone() {
                    if let Ok(mut image) = rw_image.write() {
                        if let Some((x, y)) = uv_position {
                            let img_width = canvas_state.image_width;
                            let img_height = canvas_state.image_height;
                            let px = (x * img_width as f32) as u32;
                            let py = (y * img_height as f32) as u32;

                            let radius = 5; // ペンの半径
                            for dy in -(radius as i32)..=(radius as i32) {
                                for dx in -(radius as i32)..=(radius as i32) {
                                    if dx * dx + dy * dy <= radius * radius {
                                        let nx = px as i32 + dx;
                                        let ny = py as i32 + dy;
                                        if nx >= 0
                                            && nx < img_width as i32
                                            && ny >= 0
                                            && ny < img_height as i32
                                        {
                                            // 白色で塗りつぶす
                                            let idx = (ny * 4 * img_width as i32 + nx * 4) as usize;
                                            image[idx] = 255;
                                            image[idx + 1] = 255;
                                            image[idx + 2] = 255;
                                            image[idx + 3] = 255;
                                        }
                                    }
                                }
                            }

                            canvas_state.modified_area = Some(iced::Rectangle {
                                x: (px.saturating_sub(radius as u32)) as f32,
                                y: (py.saturating_sub(radius as u32)) as f32,
                                width: (radius * 2) as f32,
                                height: (radius * 2) as f32,
                            });
                        }
                    }
                    return Status::Captured;
                }
            }
        }

        Status::Ignored
    }
}
