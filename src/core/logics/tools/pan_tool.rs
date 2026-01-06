use glam::{vec3, Quat, Vec2, Vec3};
use skia_safe::Point;
use crate::core::data::Project;
use crate::core::inputs::KeyboardInput;
use crate::core::inputs::PointerInput;
use crate::core::logics::tools::{EventHandling, Tool};
use crate::core::math::{LatLon, Radian};
use crate::core::services::{CanvasService, Services};
use crate::latlon;

pub struct PanTool {
    is_dragging: bool,
    drag_start_position: Vec2,
    drag_start_look_at: Vec3,
}

impl PanTool {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            drag_start_position: Vec2::default(),
            drag_start_look_at: Vec3::default(),
        }
    }
}

impl <S: Services> Tool<S> for PanTool {
    fn pointer_input(&mut self, input: &PointerInput, project: &mut Project, services: &mut S) -> EventHandling {
        match input {
            PointerInput::Down { button, viewport_position, uv_position } => {
                self.is_dragging = true;
                self.drag_start_position = *viewport_position;
                self.drag_start_look_at = services.canvas().look_at();
            }
            PointerInput::Move { button, viewport_position, uv_position } => {
                if self.is_dragging {
                    let viewport_bounds = services.canvas().viewport_bounds();
                    let pointer_delta = self.drag_start_position - *viewport_position;

                    let yaw = -pointer_delta.x / viewport_bounds.x;
                    let pitch = -pointer_delta.y / viewport_bounds.y;
                    let quat = Quat::from_axis_angle(services.canvas().up(), yaw)
                        .mul_quat(Quat::from_axis_angle(services.canvas().right(), -pitch));

                    let look_at = quat.mul_vec3(self.drag_start_look_at).normalize();
                    let right = calc_right(look_at);

                    services.canvas_mut().pan(look_at, right);
                }
            }
            PointerInput::Up { button, viewport_position, uv_position } => {
                self.is_dragging = false;
            }
            _ => return EventHandling::None
        }

        EventHandling::Captured
    }

    fn keyboard_input(&mut self, input: &KeyboardInput, project: &mut Project, services: &mut S) -> EventHandling {
        EventHandling::None
    }
}

fn calc_right(look_at: Vec3) -> Vec3 {
    // 視点(極座標)ベクトルの接平面の右ベクトルを求める(視点右方向のベクトル)
    // x軸との角度をxz平面で考える
    let mut phi = vec3(look_at.x, 0., look_at.z).angle_between(glam::Vec3::X);
    if look_at.z < 0. {
        phi = -phi; // z軸が負なら角度も負にする
    }
    vec3(-phi.sin(), 0., phi.cos()).normalize()
}