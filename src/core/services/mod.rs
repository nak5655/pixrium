use crate::core::services::file_dialog_service::FileDialogService;
use crate::gui::services::RfdFileDialogService;

pub mod file_dialog_service;

pub trait Services {
    type FileDialogService: FileDialogService;

    fn file_dialog(&self) -> &RfdFileDialogService;
}
