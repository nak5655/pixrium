use skia_safe::{AlphaType, Bitmap, ColorSpace, ColorType, ISize, ImageInfo};

pub struct Layer {
    pub name: String,
    pub bitmap: Bitmap,
    pub opacity: f32,
}

impl Layer {
    pub fn new(name: String, width: usize, height: usize) -> Layer {
        let image_info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBAF16,
            AlphaType::Opaque,
            Some(ColorSpace::new_srgb()),
        );

        let rowBytes: usize = width * 2;

        let mut bitmap = Bitmap::new();
        bitmap.set_info(&image_info, Some(rowBytes));

        Self {
            name,
            bitmap,
            opacity: 1.0,
        }
    }
}