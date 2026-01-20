use skia_safe::surfaces::raster_n32_premul;
use skia_safe::{ISize, Image, Paint, Rect};

pub fn resize(image: Image, size: ISize) -> Image {
    // 新しい Surface（CPU 側）を作る
    let mut surface = raster_n32_premul(size).expect("Failed to create surface");

    let canvas = surface.canvas();

    // リサイズ後の描画先
    let dst_rect = Rect::from_wh(size.width as f32, size.height as f32);

    let paint = Paint::default();

    // リサイズ描画
    canvas.draw_image_rect(image, None, dst_rect, &paint);

    // Surface → Image に変換
    surface.image_snapshot()
}
