use glam::{vec3, Vec3};

#[derive(PartialEq, Copy, Clone)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32,
}

#[macro_export]
macro_rules! latlon {
    ($lat:expr, $lon:expr) => {
        LatLon {
            lat: $lat,
            lon: $lon,
        }
    };
}

impl LatLon {
    pub fn to_unit_vec(&self) -> Vec3 {
        let cos_lat = self.lat.cos();
        vec3(
            cos_lat * self.lon.cos(),
            cos_lat * self.lon.sin(),
            self.lat.sin(),
        )
    }
}

impl From<Vec3> for LatLon {
    fn from(value: Vec3) -> Self {
        let lat = value.y.asin();
        let lon = value.z.atan2(value.x);
        LatLon { lat, lon }
    }
}
