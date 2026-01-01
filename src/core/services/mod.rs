mod dialog_service;
mod canvas_service;

pub use dialog_service::*;
pub use canvas_service::*;

pub trait Services {
    type DialogService: DialogService;
    type CanvasService: CanvasService;

    fn dialog(&self) -> &Self::DialogService;

    fn canvas(&self) -> &Self::CanvasService;

    fn canvas_mut(&mut self) -> &mut Self::CanvasService;
}
