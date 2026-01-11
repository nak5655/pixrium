mod config_service;
mod dialog_service;

pub(crate) use crate::core::services::config_service::ConfigService;
pub use config_service::*;
pub use dialog_service::*;

pub trait Services {
    type DialogService: DialogService;

    fn dialog(&self) -> &Self::DialogService;

    fn config(&self) -> &ConfigService;
}
