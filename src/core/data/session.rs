use crate::core::data::project::Project;
use crate::core::logics::{Console, Commands, OpenFileCommand};
use crate::core::services::file_dialog_service::FileDialogService;
use crate::core::services::Services;

pub struct Session {
    pub project: Option<Project>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
        }
    }
}