use crate::core::data::Session;
use crate::core::logics::Console;
use crate::core::services::Services;
use crate::gui::services::{FileDialogServiceImpl, MessageServiceImpl};
use freya::prelude::*;

mod gui;
mod core;

use crate::gui::windows::main::main_window::MainWindow;

fn main() {
    launch_with_title(FreyaApp, "Pixrium");
}

struct FreyaServices {
    file_dialog: FileDialogServiceImpl,
    message: MessageServiceImpl,
}

impl Services for FreyaServices {
    type FileDialogService = FileDialogServiceImpl;
    type MessageService = MessageServiceImpl;

    fn file_dialog(&self) -> &FileDialogServiceImpl {
        &self.file_dialog
    }

    fn message(&self) -> &MessageServiceImpl {
        &self.message
    }
}

#[component]
fn FreyaApp() -> Element {
    let console = use_signal(|| Console::new(
        Session::new(),
        FreyaServices {
            file_dialog: FileDialogServiceImpl { },
            message: MessageServiceImpl { },
        }
    ));

    rsx! {
        MainWindow { console }
    }
}