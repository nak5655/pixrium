use crate::core::logics::commands::Command;
use crate::core::logics::Console;
use crate::core::services::DialogService;
use crate::core::services::Services;

pub struct ShowVersionCommand();

impl<S: Services> Command<S> for ShowVersionCommand {
    fn execute(&self, console: &mut Console<S>) {
        console
            .services
            .dialog()
            .show_info("Version".to_string(), "Version 0.1.0".to_string());
    }
}
