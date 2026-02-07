use crate::core::data::{Layer, Project};
use crate::core::input::commands::Command;
use crate::core::logics::Console;
use crate::core::services::{DialogService, Services};
use skia_safe::Rect;

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

                    let session = console.open_session(project);

                    session.state.selected_layer_index = session.project.layers.len() - 1;
                    session.update_frame(Rect::new(
                        0.0,
                        0.0,
                        session.project.width as f32,
                        session.project.height as f32,
                    ));
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
