use crate::core::input::pointer_input::*;
use skia_safe::{Color4f, Size};

pub mod commands;
mod key;
mod keyboard_input;
mod pointer_input;
mod window_input;

use crate::core::logics::tools::Tools;
use crate::core::services::Services;
use commands::Command;
pub use key::*;
pub use keyboard_input::*;
pub use pointer_input::*;

pub enum InputSignal<S: Services> {
    Pointer(PointerInput),
    Keyboard(KeyboardInput),
    Command(Box<dyn Command<S>>),
    ChooseTool(Tools),
    ChooseColor(Color4f),
    ViewportResized(Size),
}
