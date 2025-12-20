use crate::core::services::file_dialog_service::FileDialogService;
use crate::core::services::message_service::MessageService;

pub mod file_dialog_service;
pub mod message_service;

pub trait Services {
    type FileDialogService: FileDialogService;
    type MessageService: MessageService;

    fn file_dialog(&self) -> &Self::FileDialogService;

    fn message(&self) -> &Self::MessageService;
}
