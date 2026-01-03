use crate::core::inputs::Key;

pub enum KeyboardInput {
    Down {
        key: Key,
    },
    Up {
        key: Key,
    }
}