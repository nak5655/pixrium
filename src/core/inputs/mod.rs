use skia_safe::Point;
use crate::core::inputs::pointer_input::*;

pub mod pointer_input;

pub enum Input {
    PointerInput(PointerInput),
}