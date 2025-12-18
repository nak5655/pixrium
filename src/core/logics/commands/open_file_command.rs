use freya::prelude::{use_context, GlobalSignal};
use crate::core::data::Session;
use crate::core::logics::commands::Command;
use crate::core::services::file_dialog_service::FileDialogService;
use crate::core::services::Services;

pub struct OpenFileCommand();

impl<S: Services> Command<S> for OpenFileCommand {
    fn run(&self, services: &S, session: &mut Session)  {
        services.file_dialog().OpenImageDialog();
    }
}