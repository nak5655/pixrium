use crate::core::services::message_service::MessageService;
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use std::thread::spawn;

pub struct MessageServiceImpl {

}

impl MessageServiceImpl {
    fn show_message(&self, title: String, message: String, level: MessageLevel) {
        spawn(move || {
            let _ = MessageDialog::new()
                .set_title(title)
                .set_description(message)
                .set_level(level)
                .set_buttons(MessageButtons::Ok)
                .show();
        });
    }
}

impl MessageService for MessageServiceImpl {
    fn show_info(&self, title: String, message: String) {
        self.show_message(title, message, MessageLevel::Info)
    }

    fn show_warning(&self, title: String, message: String) {
        self.show_message(title, message, MessageLevel::Warning)
    }

    fn show_error(&self, title: String, message: String) {
        self.show_message(title, message, MessageLevel::Error)
    }

    fn show_exception(&self, title: String, message: String) {
        self.show_message(title, message, MessageLevel::Error)
    }
}