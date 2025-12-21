use skia_safe::image::CachingHint;
use skia_safe::{AlphaType, Bitmap, ColorSpace, ColorType, Data, ISize, Image, ImageInfo};
use std::path::PathBuf;

pub fn load_bitmap(path: PathBuf, color_type: ColorType, alpha_type: AlphaType, color_space: Option<ColorSpace>) -> Result<Bitmap, String> {
    let data = Data::new_copy(std::fs::read(path.clone()).unwrap().as_slice());

    // make image
    let image = match Image::from_encoded(data) {
        Some(image) => image,
        None => return Err("failed to decode image.".into()),
    };

    let image_info = ImageInfo::new(
        ISize::new(image.width() as i32, image.height() as i32),
        color_type,
        alpha_type,
        color_space,
    );

    // make bitmap
    let mut bitmap = Bitmap::new();
    bitmap.alloc_pixels_info(&image_info, None);

    // make pixmap
    let mut pixmap = match bitmap.peek_pixels() {
        Some(pixmap) => pixmap,
        None => return Err("failed to peek pixels.".into()),
    };
    let pixels = match pixmap.bytes_mut() {
        Some(pixels) => pixels,
        None => return Err("failed to read bytes of pixels.".into()),
    };

    // image to pixmap
    image.read_pixels(&image_info, pixels, bitmap.row_bytes(), (0, 0), CachingHint::Disallow);

    Ok(bitmap)
}