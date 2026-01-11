use crate::core::data::Session;
use crate::core::logics::commands::Command;
use crate::core::services::Services;

pub struct ChangeLayerOpacityCommand {
    pub opacity: f32,
}

impl<S: Services> Command<S> for ChangeLayerOpacityCommand {
    fn execute(&self, _: &S, session: &mut Session) {
        if let Some(layer) = session.selected_layer_mut() {
            layer.opacity = self.opacity;
        }
    }
}
