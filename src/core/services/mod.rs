mod dialog_service;
mod canvas_service;
mod config_service;

pub use dialog_service::*;
pub use canvas_service::*;
pub use config_service::*;
pub(crate) use crate::core::services::config_service::ConfigService;

pub trait Services {
    type DialogService: DialogService;
    type CanvasService: CanvasService;

    fn dialog(&self) -> &Self::DialogService;

    fn canvas(&self) -> &Self::CanvasService;

    fn canvas_mut(&mut self) -> &mut Self::CanvasService;

    fn config(&self) -> &ConfigService;
}
