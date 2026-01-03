use skia_safe::Point;
use crate::core::inputs::pointer_input::*;

mod pointer_input;
mod keyboard_input;
mod key;

pub use key::*;
pub use pointer_input::*;
pub use keyboard_input::*;

pub enum Input {
    Pointer(PointerInput),
    Keyboard(KeyboardInput)
}