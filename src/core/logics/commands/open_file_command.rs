use crate::core::data::{Layer, Project, Session};
use crate::core::logics::commands::Command;
use crate::core::services::file_dialog_service::FileDialogService;
use crate::core::services::message_service::MessageService;
use crate::core::services::Services;

pub struct OpenFileCommand();

impl<S: Services> Command<S> for OpenFileCommand {
    fn execute(&self, services: &S, session: &mut Session)  {
        match services.file_dialog().open_image() {
            Some(image_path) => {
                match Layer::load(image_path) {
                    Ok(layer) => {
                        // プロジェクトが開かれていない場合作成(暫定仕様)
                        if session.project.is_none() {
                            session.project = Some(Project::new(layer.bitmap.width() as usize, layer.bitmap.height() as usize))
                        }
                        if let Some(project) = &mut session.project {
                            project.layers.push(layer);

                            // 作成したレイヤーを選択
                            session.selected_layer_index = Some(project.layers.len() - 1);
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