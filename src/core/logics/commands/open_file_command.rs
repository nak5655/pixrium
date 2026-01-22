use crate::core::data::{Layer, Project, Session};
use crate::core::logics::commands::Command;
use crate::core::logics::Console;
use crate::core::services::{DialogService, Services};

pub struct OpenFileCommand();

impl<S: Services> Command<S> for OpenFileCommand {
    fn execute(&self, console: &mut Console<S>) {
        match console.services.dialog().open_image() {
            Some(image_path) => match Layer::load(image_path) {
                Ok(layer) => {
                    let mut project = Project::new(
                        layer.bitmap.width() as usize,
                        layer.bitmap.height() as usize,
                    );
                    project.layers.push(layer);

                    let mut session = Session::new(project);
                    session.selected_layer_index = session.project.layers.len() - 1;
                    session.update_preview();

                    console.session = Some(session);
                }
                Err(err) => {
                    console
                        .services
                        .dialog()
                        .show_error("An error occured".to_string(), err);
                }
            },
            None => return,
        }
    }
}
