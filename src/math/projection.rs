use std::f32::consts::PI;

use glam::{Mat3, Vec2, Vec3, vec2, vec3};

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
     * view座標 (-1.0 ... 1.0) をテクスチャ座標 (-1.0 ... 1.0) に射影する
     */
    pub fn proj(&self, view_x: f32, view_y: f32) -> Vec2 {
        // Calculate yaw and pitch
        let yaw = self.aov * (view_x - 0.5);
        let pitch = self.aov * (view_y - 0.5);

        // Calculate sphere coordinates
        let sphere_coord = (self.look_at + yaw * self.right + pitch * self.up).normalize();

        // Calculate spherical coordinates
        // XZ平面への射影角度(phi)とY軸からの偏角(theta)
        let phi: f32 = sphere_coord.z.atan2(sphere_coord.x);
        let theta = sphere_coord
            .y
            .atan2((sphere_coord.x.powi(2) + sphere_coord.z.powi(2)).sqrt());

        // Convert to texture UV
        let tex_u = phi / (2.0 * PI) + 0.5;
        let tex_v = 0.5 - theta / PI;
        vec2(tex_u, tex_v)
    }

    /**
     * view座標 (-1.0 ... 1.0) をテクスチャ座標 (-1.0 ... 1.0) に逆射影する
     * (線形化射影を用いる場合)
     */
    // pub fn unproj(&self, tex_u: f32, tex_v: f32) -> Vec2 {
    //     // Convert texture UV back to spherical angles
    //     let phi = (tex_u - 0.5) * 2.0 * PI; // azimuth
    //     let theta = (tex_v - 0.5) * PI; // elevation

    //     // Reconstruct direction on the unit sphere from spherical angles
    //     let sphere_coord = vec3(
    //         theta.cos() * phi.cos(),
    //         theta.sin(),
    //         theta.cos() * phi.sin(),
    //     );

    //     // Project onto the local right/up axes to recover yaw and pitch (linearized)
    //     let yaw = (sphere_coord - self.look_at).dot(self.right);
    //     let pitch = (sphere_coord - self.look_at).dot(self.up);

    //     // Convert back to view coordinates
    //     let view_x = yaw / self.aov + 0.5;
    //     let view_y = pitch / self.aov + 0.5;

    //     vec2(view_x, view_y)
    // }

    /**
     * テクスチャ座標 (-1.0 ... 1.0) を view座標 (-1.0 ... 1.0) に逆射影する
     * (線形化射影ではなく厳密解を求める)
     */
    pub fn unproj(&self, tex_u: f32, tex_v: f32) -> Vec2 {
        // Convert texture UV back to spherical angles (inverse of proj)
        let phi = (tex_u - 0.5) * 2.0 * PI; // azimuth
        let theta = (0.5 - tex_v) * PI; // elevation (note the sign to invert tex_v = 0.5 - theta/PI)

        // Reconstruct direction on the unit sphere from spherical angles
        let sphere_dir = vec3(
            theta.cos() * phi.cos(),
            theta.sin(),
            theta.cos() * phi.sin(),
        );

        // Solve for yaw, pitch and scale s:
        // look_at + yaw * right + pitch * up = s * sphere_dir
        // => [ right | up | -sphere_dir ] * [yaw, pitch, s]^T = -look_at
        let mat = Mat3::from_cols(self.right, self.up, -sphere_dir);
        let b = -self.look_at;

        let eps = 1e-6;
        if mat.determinant().abs() > eps {
            let sol = mat.inverse() * b;
            let yaw = sol.x;
            let pitch = sol.y;
            let view_x = yaw / self.aov + 0.5;
            let view_y = pitch / self.aov + 0.5;
            vec2(view_x, view_y)
        } else {
            // Fallback to linearized projection (approximate)
            let yaw = (sphere_dir - self.look_at).dot(self.right);
            let pitch = (sphere_dir - self.look_at).dot(self.up);
            let view_x = yaw / self.aov + 0.5;
            let view_y = pitch / self.aov + 0.5;
            vec2(view_x, view_y)
        }
    }
}
