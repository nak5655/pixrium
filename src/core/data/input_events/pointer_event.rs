use skia_safe::Point;

pub enum PointerButton {
    Left,
    Right,
    Middle,
}

pub enum PointerEvent {
    Down {
        button: PointerButton,
        viewport_position: Point,
        uv_position: Point,
    },
    Move {
        viewport_position: Point,
        uv_position: Point,
    },
    Up {
        button: PointerButton,
        viewport_position: Point,
        uv_position: Point,
    },
}
