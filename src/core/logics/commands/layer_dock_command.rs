use crate::core::data::Session;
use crate::core::logics::commands::Command;
use crate::core::services::Services;

pub struct ChangeLayerOpacityCommand {
    pub opacity: f32,
}

impl<S: Services> Command<S> for ChangeLayerOpacityCommand {
    fn execute(&self, _: &S, session: &mut Session)  {
        if let Some(selected_layer_index) = session.selected_layer_index {
            if let Some(project) = &mut session.project {
                if selected_layer_index < project.layers.len() {
                    if let Some(layer) = project.layers.get_mut(selected_layer_index) {
                        layer.opacity = self.opacity;
                    }
                }
            }
        }
    }
}