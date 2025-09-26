#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Move,
    Object,
    Node,
    Select,
    Pen,
    Brush,
    Eraser,
    Zoom,
}