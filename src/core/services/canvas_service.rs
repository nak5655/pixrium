use glam::{Vec2, Vec3};
use crate::core::math::{LatLon, Radian};

pub trait CanvasService {
    fn look_at(&self) -> Vec3;
    
    fn right(&self) -> Vec3;
    
    fn up(&self) -> Vec3;

    fn viewport_bounds(&self) -> Vec2;

    #[allow(unused)]
    fn pan(&mut self, look_at: Vec3, right: Vec3);

    #[allow(unused)]
    fn zoom(&mut self, fov: Radian);
}