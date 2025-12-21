use crate::core::data::Session;
use crate::core::logics::commands::Command;
use crate::core::services::message_service::MessageService;
use crate::core::services::Services;

pub struct ShowVersionCommand();

impl<S: Services> Command<S> for ShowVersionCommand {
    fn execute(&self, services: &S, _: &mut Session)  {
        services.message().show_info("Version".to_string(), "Version 0.1.0".to_string());
    }
}