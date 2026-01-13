use crate::core::data::Session;
use crate::core::logics::tools::Tool;
use crate::core::logics::Console;
use crate::core::services::ConfigService;
use crate::core::services::Services;
use crate::gui::services::DialogServiceImpl;
use freya::prelude::*;

mod core;
mod gui;

use crate::gui::windows::main::main_window;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_title("Pixrium")))
}

struct FreyaServices {
    dialog_service: DialogServiceImpl,
    config_service: ConfigService,
}

impl Services for FreyaServices {
    type DialogService = DialogServiceImpl;

    fn dialog(&self) -> &Self::DialogService {
        &self.dialog_service
    }

    fn config(&self) -> &ConfigService {
        &self.config_service
    }
}

fn app() -> impl IntoElement {
    let mut session = Session::new();

    let services = FreyaServices {
        dialog_service: DialogServiceImpl {},
        config_service: ConfigService::new(),
    };

    let console = use_state(|| Console::new(session, services));

    rect().center().expanded().child(main_window(console))
}
