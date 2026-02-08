use crate::core::input::commands::Command;
use crate::core::logics::Console;
use crate::core::services::{DialogService, Services};

pub struct OpenFileCommand();

impl<S: Services> Command<S> for OpenFileCommand {
    fn execute(&self, console: &mut Console<S>) {
        match console.services.dialog().open_image() {
            Some(image_path) => match console.open_session_from_image(image_path) {
                Err(err) => {
                    console
                        .services
                        .dialog()
                        .show_error("An error occured".to_string(), err);
                }
                _ => (),
            },
            None => return,
        }
    }
}
