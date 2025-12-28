use std::collections::HashMap;
use crate::core::data::Session;
use crate::core::logics::Console;
use crate::core::services::Services;
use crate::gui::services::{FileDialogServiceImpl, MessageServiceImpl};
use freya::prelude::*;
use crate::core::logics::tools::{PanTool, Tool, Tools};

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
    let services = FreyaServices {
        file_dialog: FileDialogServiceImpl { },
        message: MessageServiceImpl { },
    };

    let mut tools: HashMap<Tools, Box<dyn Tool>> = HashMap::new();
    tools.insert(Tools::Pan, Box::new(PanTool::new()));

    let session = Session::new(tools);

    let console = use_signal(|| Console::new(
        session,
        services
    ));

    rsx! {
        MainWindow { console }
    }
}