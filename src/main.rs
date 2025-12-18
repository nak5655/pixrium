use freya::prelude::*;
use crate::core::data::Session;
use crate::core::data::Project;
use crate::core::logics::Console;
use crate::core::services::Services;
use crate::gui::services::RfdFileDialogService;

mod gui;
mod core;

use crate::gui::windows::main::main_window::MainWindow;

fn main() {
    launch(FreyaApp);
}

struct FreyaServices {
    file_dialog: RfdFileDialogService,
}

impl Services for FreyaServices {
    type FileDialogService = RfdFileDialogService;

    fn file_dialog(&self) -> &RfdFileDialogService {
        &self.file_dialog
    }
}

#[component]
fn FreyaApp() -> Element {
    let console = use_signal(|| Console::new(
        Session::new(),
        FreyaServices {
            file_dialog: RfdFileDialogService { },
        }
    ));

    rsx! {
        MainWindow { console }
    }
}