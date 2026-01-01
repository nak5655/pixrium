use crate::core::services::DialogService;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};
use std::path::PathBuf;
use std::thread::spawn;

pub struct DialogServiceImpl {

}

impl DialogServiceImpl {
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

impl DialogService for DialogServiceImpl {
    fn open_image(&self) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("HDR image", &["hdr"])
            .add_filter("image", &["jpg", "png"])
            .set_directory("/")
            .pick_file()
    }

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

