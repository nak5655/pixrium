use glam::Vec2;

#[derive(Copy, Clone)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
}

pub enum PointerInput {
    Down {
        button: PointerButton,
        viewport_position: Vec2,
        uv_position: Vec2,
    },
    Move {
        button: PointerButton,
        viewport_position_delta: Vec2,
        viewport_position: Vec2,
        uv_position_delta: Vec2,
        uv_position: Vec2,
    },
    Up {
        button: PointerButton,
        viewport_position: Vec2,
        uv_position: Vec2,
    },
    Scroll {
        delta: Vec2,
        viewport_position: Vec2,
        uv_position: Vec2,
    },
}
