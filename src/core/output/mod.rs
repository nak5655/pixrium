use crate::core::data::ViewportState;
use skia_safe::Image;

pub enum OutputSignal {
    Frame(Image),
    Viewport(ViewportState),
}
