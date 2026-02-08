use crate::core::data::Session;
use crate::core::input::{KeyboardInput, PointerButton, PointerInput};
use crate::core::logics::tools::{EventHandling, Tool};
use crate::core::math::SphereProjection;
use crate::core::services::Services;
use glam::{vec2, Vec2, Vec3};
use skia_safe::canvas::PointMode;
use skia_safe::{scalar, Canvas, Color, Paint, Point, Rect};
use std::collections::{HashSet, VecDeque};
use std::f32::consts::PI;

pub struct BrushTool {
    width: f32,
    color: Color,
    is_dragging: bool,
    drag_start_position: Vec2,
    drag_start_look_at: Vec3,
}

impl BrushTool {
    pub fn new() -> Self {
        Self {
            width: 3.0,
            color: Color::RED,
            is_dragging: false,
            drag_start_position: Vec2::default(),
            drag_start_look_at: Vec3::default(),
        }
    }
}

impl<S: Services> Tool<S> for BrushTool {
    fn pointer_input(
        &mut self,
        input: &PointerInput,
        session: &mut Session,
        _services: &mut S,
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
                button,
                viewport_position,
            } => {
                let tex_w = session.project.width as i32;
                let tex_h = session.project.height as i32;

                // Get the UV position before acquiring mutable borrow
                let mp = viewport_position
                    / vec2(
                        session.state.viewport.size.width as f32,
                        session.state.viewport.size.width as f32,
                    ); //session.viewport_bounds();
                let pixel_scale = session.state.viewport.fov / 2.0 / PI * tex_w as f32;

                // while press the left _button
                if Some(PointerButton::Left) != *button {
                    return EventHandling::Ignored;
                }

                if let Some(layer) = session.selected_layer_mut() {
                    let canvas = Canvas::from_bitmap(&mut layer.bitmap, None).unwrap();

                    // view座標(0.0~1.0)からテクスチャ座標(-1.0~1.0)への射影関数
                    let proj = SphereProjection::new(
                        session.state.viewport.fov,
                        session.state.viewport.look_at,
                        session.state.viewport.up(),
                        session.state.viewport.right,
                    );

                    // テクスチャのピクセルでの中心座標
                    let tex_cp = proj.proj(mp.x, mp.y);
                    let tex_cx = (tex_cp.x * tex_w as f32).round() as i32;
                    let tex_cy = (tex_cp.y * tex_h as f32).round() as i32;
                    // 距離計測の基準点を再計算（計算誤差を考慮）
                    let cp = proj.unproj(tex_cp.x, tex_cp.y);

                    // 走査予定のピクセル
                    let mut rest = VecDeque::new();
                    rest.push_back((tex_cx, tex_cy));

                    // 走査済みのピクセル
                    let mut visited = HashSet::new();
                    let mut min_x = tex_w;
                    let mut max_x = 0;
                    let mut min_y = tex_h;
                    let mut max_y = 0;

                    // ピクセルの走査
                    let mut points = vec![];
                    while rest.len() > 0 {
                        let (px, py) = rest.pop_front().unwrap();

                        // viewでの距離を求める
                        let u = px as f32 / tex_w as f32;
                        let v = py as f32 / tex_h as f32;
                        let vp = proj.unproj(u, v);

                        let dx = vp.x - cp.x;
                        let dy = vp.y - cp.y;
                        let distance2 = dx * dx + dy * dy;

                        let radius = self.width / pixel_scale;

                        min_x = min_x.min(px);
                        max_x = max_x.max(px);
                        min_y = min_y.min(py);
                        max_y = max_y.max(py);

                        if distance2 <= radius * radius {
                            // 塗りつぶす
                            if px >= 0 && px < tex_w as i32 && py >= 0 && py < tex_h as i32 {
                                points.push(Point::new(px as scalar, py as scalar));
                            }

                            // 隣接ピクセルを追加
                            for (nx, ny) in [(px + 1, py), (px - 1, py), (px, py + 1), (px, py - 1)]
                            {
                                if nx >= 0 && nx < tex_w as i32 && ny >= 0 && ny < tex_h as i32 {
                                    if visited.contains(&(nx, ny)) {
                                        continue;
                                    }
                                    rest.push_back((nx, ny));
                                    visited.insert((nx, ny));
                                }
                            }
                        }
                    }

                    // draw
                    let mut paint = Paint::default();
                    paint.set_color(self.color);
                    paint.set_anti_alias(false);
                    paint.set_alpha(255);
                    paint.set_alpha_f(1.0);

                    canvas.draw_points(PointMode::Points, points.as_slice(), &paint);

                    //// テクスチャの更新範囲
                    session.update_frame(Rect::new(
                        min_x as f32,
                        min_y as f32,
                        max_x as f32,
                        max_y as f32,
                    ));
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
        _input: &KeyboardInput,
        _session: &mut Session,
        _services: &mut S,
    ) -> EventHandling {
        EventHandling::Ignored
    }
}
