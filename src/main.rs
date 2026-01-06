use std::collections::HashMap;
use crate::core::data::{Project, Session};
use crate::core::logics::Console;
use crate::core::services::Services;
use crate::core::services::ConfigService;
use crate::gui::services::{CanvasServiceImpl, DialogServiceImpl};
use freya::prelude::*;
use crate::core::logics::tools::{PanTool, Tool, Tools};
use crate::gui::components::canvas::CanvasState;

mod gui;
mod core;

use crate::gui::windows::main::main_window;

fn main() {
    launch_with_title(app, "Pixrium")
}

struct FreyaServices {
    dialog_service: DialogServiceImpl,
    canvas_service: CanvasServiceImpl,
    config_service: ConfigService,
}

impl Services for FreyaServices {
    type DialogService = DialogServiceImpl;
    type CanvasService = CanvasServiceImpl;

    fn dialog(&self) -> &Self::DialogService {
        &self.dialog_service
    }

    fn canvas(&self) -> &Self::CanvasService {
        &self.canvas_service
    }

    fn canvas_mut(&mut self) -> &mut Self::CanvasService {
        &mut self.canvas_service
    }

    fn config(&self) -> &ConfigService {
        &self.config_service
    }
}

#[component]
fn app() -> Element {
    let canvas_state = use_signal(|| CanvasState::new());

    let services = FreyaServices {
        dialog_service: DialogServiceImpl {},
        canvas_service: CanvasServiceImpl {
            canvas_state
        },
        config_service: ConfigService::new(),
    };

    let mut session = Session::new();
    session.project = Some(Project::new(1, 1));

    let console = use_signal(|| Console::new(
        session,
        services,
    ));

    rsx! {
        rect {
            main_window {
                console,
                canvas_state,
            }
        }
    }
}