use std::sync::{Arc, RwLock};

use glam::Quat;
use iced::advanced::graphics::core::event::Status;
use iced::mouse;

use crate::tool::Tool;
use crate::widget::sphere_canvas::SphereCanvasState;

#[derive(Debug)]
pub struct PanTool {
    pub name: String,
    pub icon: char,
}

impl PanTool {
    pub fn new() -> Self {
        Self {
            name: "Pan".to_string(),
            icon: '\u{ec2e}',
        }
    }
}

impl Tool for PanTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn icon(&self) -> char {
        self.icon
    }

    fn on_mouse_moved(&self, canvas_state: &Arc<RwLock<SphereCanvasState>>) -> Status {
        if let Ok(mut canvas_state) = canvas_state.try_write() {
            if canvas_state.mouse_button == Some(mouse::Button::Middle) {
                let yaw = canvas_state.mouse_delta.x / canvas_state.viewport_bounds.width;
                let pitch = -canvas_state.mouse_delta.y / canvas_state.viewport_bounds.width;
                let quat = Quat::from_axis_angle(canvas_state.up, yaw)
                    .mul_quat(Quat::from_axis_angle(canvas_state.right, -pitch));

                canvas_state.look_at = quat.mul_vec3(canvas_state.look_at).normalize();

                // 視点(極座標)ベクトルの接平面の右ベクトルを求める(視点右方向のベクトル)
                // x軸との角度をxz平面で考える
                let mut phi = glam::vec3(canvas_state.look_at.x, 0., canvas_state.look_at.z)
                    .angle_between(glam::Vec3::X);
                if canvas_state.look_at.z < 0. {
                    phi = -phi; // z軸が負なら角度も負にする
                }
                canvas_state.right = glam::vec3(-phi.sin(), 0., phi.cos()).normalize();
                // 接平面の上ベクトルは視点ベクトルから見て右ベクトルと直交する
                canvas_state.up = canvas_state.right.cross(canvas_state.look_at).normalize();

                return Status::Captured;
            }
        }

        Status::Ignored
    }
}
