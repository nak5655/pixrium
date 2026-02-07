use glam::Vec2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PointerInput {
    Down {
        button: PointerButton,
        viewport_position: Vec2,
    },
    Move {
        button: Option<PointerButton>,
        viewport_position: Vec2,
    },
    Up {
        button: PointerButton,
        viewport_position: Vec2,
    },
    Scroll {
        delta: Vec2,
        viewport_position: Vec2,
    },
}
