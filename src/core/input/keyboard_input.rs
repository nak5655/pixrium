use crate::core::input::Key;

pub enum KeyboardInput {
    Down { key: Key },
    Up { key: Key },
}
