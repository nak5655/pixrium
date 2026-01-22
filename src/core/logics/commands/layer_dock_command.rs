use crate::core::logics::commands::Command;
use crate::core::logics::Console;
use crate::core::services::Services;

pub struct ChangeLayerOpacityCommand {
    pub opacity: f32,
}

impl<S: Services> Command<S> for ChangeLayerOpacityCommand {
    fn execute(&self, console: &mut Console<S>) {
        if let Some(session) = console.session.as_mut() {
            if let Some(layer) = session.selected_layer_mut() {
                layer.opacity = self.opacity;
            }
        }
    }
}
