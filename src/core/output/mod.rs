use crate::core::data::ViewportState;
use crate::core::logics::tools::Tools;
use skia_safe::Image;

pub enum OutputSignal {
    Frame(Image),
    Viewport(ViewportState),
    ActiveTool(Tools),
}
