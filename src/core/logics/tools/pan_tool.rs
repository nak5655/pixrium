use crate::core::data::Session;
use crate::core::input::KeyboardInput;
use crate::core::input::PointerInput;
use crate::core::logics::tools::{EventHandling, Tool};
use crate::core::services::Services;
use glam::{vec3, Quat, Vec2, Vec3};

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

impl<S: Services> Tool<S> for PanTool {
    fn pointer_input(
        &mut self,
        input: &PointerInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling {
        match input {
            PointerInput::Down {
                button: _button,
                viewport_position,
            } => {
                self.is_dragging = true;
                self.drag_start_position = *viewport_position;
                self.drag_start_look_at = session.state.viewport.look_at;
            }
            PointerInput::Move {
                button: _button,
                viewport_position,
            } => {
                if self.is_dragging {
                    let viewport_size = session.state.viewport.size;
                    let pointer_delta = viewport_position - self.drag_start_position;

                    let yaw = pointer_delta.x / viewport_size.x;
                    let pitch = pointer_delta.y / viewport_size.y;
                    let quat = Quat::from_axis_angle(session.state.viewport.up(), yaw)
                        .mul_quat(Quat::from_axis_angle(session.state.viewport.right, -pitch));

                    let look_at = quat.mul_vec3(self.drag_start_look_at).normalize();
                    let right = calc_right(look_at);

                    session.state_mut().viewport.look_at = look_at;
                    session.state_mut().viewport.right = right;

                    session.update_viewport();
                }
            }
            PointerInput::Up {
                button: _button,
                viewport_position: _viewport_position,
            } => {
                self.is_dragging = false;
            }
            _ => return EventHandling::Ignored,
        }

        EventHandling::Captured
    }

    fn keyboard_input(
        &mut self,
        input: &KeyboardInput,
        session: &mut Session,
        services: &mut S,
    ) -> EventHandling {
        EventHandling::Ignored
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
