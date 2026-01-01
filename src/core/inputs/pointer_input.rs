use skia_safe::Point;
use crate::core::inputs::Input;

#[derive(Copy, Clone)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
}

pub enum PointerInput {
    Down {
        button: PointerButton,
        viewport_position: Point,
        uv_position: Point,
    },
    Move {
        button: PointerButton,
        viewport_position: Point,
        uv_position: Point,
    },
    Up {
        button: PointerButton,
        viewport_position: Point,
        uv_position: Point,
    }
}