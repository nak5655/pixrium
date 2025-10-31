use std::collections::{HashSet, VecDeque};
use std::f32::consts::PI;
use std::sync::{Arc, RwLock};

use iced::advanced::graphics::core::event::Status;
use iced::mouse;
use image::Rgba;

use crate::math::projection::SphereProjection;
use crate::tool::Tool;
use crate::widget::sphere_canvas::SphereCanvasState;

#[derive(Debug)]
pub struct PenTool {
    pub name: String,
    pub icon: char,

    pub width: f32,
    pub color: Rgba<u8>,
}

impl PenTool {
    pub fn new() -> Self {
        Self {
            name: "Pen".to_string(),
            icon: '\u{eb04}',

            width: 3.0,
            color: Rgba([255, 255, 255, 255]),
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
        let mut cp;
        // viewに収まる最小ピクセル数
        let pixel_scale;

        if let Ok(canvas_state) = canvas_state.try_read() {
            cp = canvas_state.get_mouse_coord_in_view();
            pixel_scale = canvas_state.aov / 2.0 / PI * canvas_state.image_width as f32;
        } else {
            return Status::Ignored;
        };

        if let Ok(mut canvas_state) = canvas_state.try_write() {
            if canvas_state.mouse_button == Some(mouse::Button::Left) {
                if let Some(rw_image) = canvas_state.image.clone() {
                    if let Ok(mut image) = rw_image.write() {
                        let tex_w = canvas_state.image_width as i32;
                        let tex_h = canvas_state.image_height as i32;

                        // view座標(0.0~1.0)からテクスチャ座標(-1.0~1.0)への射影関数
                        let proj = SphereProjection::new(
                            canvas_state.aov,
                            canvas_state.look_at,
                            canvas_state.up,
                            canvas_state.right,
                        );

                        // テクスチャのピクセルでの中心座標
                        let tex_cp = proj.proj(cp.x, cp.y);
                        let tex_cx = (tex_cp.x * tex_w as f32).round() as i32;
                        let tex_cy = (tex_cp.y * tex_h as f32).round() as i32;
                        // 距離計測の基準点を再計算（計算誤差を考慮）
                        cp =
                            proj.unproj(tex_cx as f32 / tex_w as f32, tex_cy as f32 / tex_h as f32);

                        // 塗りつぶし予定のピクセル
                        let mut rest = VecDeque::new();
                        rest.push_back((tex_cx, tex_cy));

                        // 走査済みのピクセル
                        let mut visited = HashSet::new();
                        let mut min_x = tex_w;
                        let mut max_x = 0;
                        let mut min_y = tex_h;
                        let mut max_y = 0;

                        // ピクセルの走査
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
                                // 白色で塗りつぶす
                                if px >= 0 && px < tex_w as i32 && py >= 0 && py < tex_h as i32 {
                                    image.put_pixel(px as u32, py as u32, self.color);
                                }

                                // 隣接ピクセルを追加
                                for (nx, ny) in
                                    [(px + 1, py), (px - 1, py), (px, py + 1), (px, py - 1)]
                                {
                                    if nx >= 0 && nx < tex_w as i32 && ny >= 0 && ny < tex_h as i32
                                    {
                                        if visited.contains(&(nx, ny)) {
                                            continue;
                                        }
                                        rest.push_back((nx, ny));
                                        visited.insert((nx, ny));
                                    }
                                }
                            }
                        }

                        // テクスチャの更新範囲
                        // TODO: 現状無視して全範囲更新する
                        canvas_state.modified_area = Some(iced::Rectangle {
                            x: min_x as f32 as f32,
                            y: min_y as f32 / tex_h as f32,
                            width: (max_x - min_x + 1) as f32 / tex_w as f32,
                            height: (max_y - min_y + 1) as f32 / tex_h as f32,
                        });
                    }
                    return Status::Captured;
                }
            }
        }

        Status::Ignored
    }
}
