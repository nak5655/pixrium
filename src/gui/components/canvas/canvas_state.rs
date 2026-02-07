use glam::Vec3;
use skia_safe::Image;

#[derive(Clone, Default)]
pub struct CanvasState {
    pub frame: Option<Image>,
    pub look_at: Vec3,
    pub right: Vec3,
    pub fov: f32,
}

impl PartialEq for CanvasState {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.is_some() && other.frame.is_some() {
            if self.frame.as_ref().unwrap().unique_id() != other.frame.as_ref().unwrap().unique_id()
            {
                return false;
            }
        }
        if self.look_at != other.look_at {
            return false;
        }
        return self.fov == other.fov;
    }
}
