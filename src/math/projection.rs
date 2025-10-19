use std::f32::consts::PI;

use glam::{Vec2, Vec3, vec2, vec3};

pub struct SphereProjection {
    pub aov: f32,
    pub look_at: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}

impl SphereProjection {
    pub fn new(aov: f32, look_at: Vec3, up: Vec3, right: Vec3) -> Self {
        Self {
            aov,
            look_at,
            up,
            right,
        }
    }

    /**
     * view座標 (0.0 ... 1.0) をテクスチャ座標 (0.0 ... 1.0) に射影する
     */
    pub fn proj(&self, view_x: f32, view_y: f32) -> Vec2 {
        // Calculate yaw and pitch
        let yaw = self.aov * (view_x - 0.5);
        let pitch = self.aov * (view_y - 0.5);

        // Calculate sphere coordinates
        let sphere_coord = (self.look_at + yaw * self.right + pitch * self.up).normalize();

        // Calculate spherical coordinates
        let x = sphere_coord.z.atan2(sphere_coord.x);
        let y = sphere_coord
            .y
            .atan2((sphere_coord.x.powi(2) + sphere_coord.z.powi(2)).sqrt());

        // Convert to texture UV
        let tex_u = x / (2.0 * PI) + 0.5;
        let tex_v = y / PI + 0.5;
        vec2(tex_u, tex_v)
    }

    pub fn unproj(&self, tex_u: f32, tex_v: f32) -> Vec2 {
        // Convert texture UV to spherical coordinates
        let x = (tex_u - 0.5) * (2.0 * PI);
        let y = (0.5 - tex_v) * PI;

        // Calculate sphere coordinates
        let sphere_x = y.cos() * x.cos();
        let sphere_y = y.sin();
        let sphere_z = y.cos() * x.sin();
        let sphere_coord = vec3(sphere_x, sphere_y, sphere_z).normalize();

        // Calculate yaw and pitch
        let yaw = sphere_coord.angle_between(self.look_at);
        let pitch = sphere_coord.angle_between(self.up);

        // Convert to view coordinates
        let view_x = yaw / self.aov + 0.5;
        let view_y = pitch / self.aov + 0.5;
        vec2(view_x, view_y)
    }
}
