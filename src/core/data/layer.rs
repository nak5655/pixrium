use crate::core::utils::{load_bitmap, resize};
use skia_safe::images::raster_from_bitmap;
use skia_safe::{AlphaType, Bitmap, Canvas, ColorSpace, ColorType, ISize, Image, ImageInfo};
use std::path::PathBuf;

pub struct Layer {
    pub name: String,
    pub bitmap: Bitmap,
    pub opacity: f32,
    pub thumbnail: Image,
}

impl Layer {
    fn color_type() -> ColorType {
        ColorType::RGBAF16
    }

    fn alpha_type() -> AlphaType {
        AlphaType::Opaque
    }

    fn color_space() -> Option<ColorSpace> {
        Some(ColorSpace::new_srgb())
    }

    pub fn new(name: String, width: usize, height: usize) -> Self {
        let row_bytes: usize = width * 2;

        let image_info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            Self::color_type(),
            Self::alpha_type(),
            Self::color_space(),
        );

        let mut bitmap = Bitmap::new();
        let _ = bitmap.set_info(&image_info, Some(row_bytes));

        let mut canvas = Canvas::from_bitmap(&bitmap, None).unwrap();

        let thumbnail = resize(raster_from_bitmap(&bitmap).unwrap(), ISize::new(256, 128));

        Self {
            name,
            bitmap,
            opacity: 1.0,
            thumbnail,
        }
    }

    pub fn load(path: PathBuf) -> Result<Self, String> {
        let bitmap = match load_bitmap(
            path.clone(),
            Self::color_type(),
            Self::alpha_type(),
            Self::color_space(),
        ) {
            Ok(bitmap) => bitmap,
            Err(e) => return Err(e),
        };

        let thumbnail = resize(raster_from_bitmap(&bitmap).unwrap(), ISize::new(256, 128));

        Ok(Self {
            name: path
                .file_name()
                .and_then(|os_str| os_str.to_os_string().into_string().ok())
                .unwrap_or("no name".into()),
            bitmap,
            opacity: 1.0,
            thumbnail,
        })
    }
}
