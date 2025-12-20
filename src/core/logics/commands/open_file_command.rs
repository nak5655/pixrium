use crate::core::data::{Layer, Session};
use crate::core::logics::commands::Command;
use crate::core::services::file_dialog_service::FileDialogService;
use crate::core::services::message_service::MessageService;
use crate::core::services::Services;

pub struct OpenFileCommand();

impl<S: Services> Command<S> for OpenFileCommand {
    fn run(&self, services: &S, session: &mut Session)  {
        match (services.file_dialog().open_image()) {
            Some(image_path) => {
                match (Layer::load(image_path)) {
                    Ok(layer) => {
                        if let Some(project) = &mut session.project {
                            project.layers.push(layer);
                        }
                    }
                    Err(err) => {
                        services.message().show_error("An error occured".to_string(), err);
                    }
                }

            }
            None => return
        }

    }
}