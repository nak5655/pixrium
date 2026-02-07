use crate::core::logics::tools::Tool;
use crate::core::logics::Console;
use crate::core::output::OutputSignal;
use crate::core::services::ConfigService;
use crate::core::services::Services;
use crate::gui::components::canvas::CanvasState;
use crate::gui::components::ToolbarState;
use crate::gui::services::DialogServiceImpl;
use freya::prelude::*;
use freya_radio::prelude::{use_init_radio_station, use_radio, RadioChannel};
use std::sync::mpsc;

mod core;
mod gui;

use crate::gui::windows::main::{main_window, MainState};

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

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum OutputChannel {
    Canvas,
    Toolbar,
}

impl RadioChannel<MainState> for OutputChannel {}

fn app() -> impl IntoElement {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    spawn(async move {
        let services = FreyaServices {
            dialog_service: DialogServiceImpl {},
            config_service: ConfigService::new(),
        };
        let mut console = Console::new(services, output_tx);

        let mut ticker = consume_root_context::<RenderingTicker>();

        loop {
            console.update(&input_rx);

            ticker.tick().await;
        }
    });

    let mut input_tx_state = use_state(move || input_tx);

    use_init_radio_station::<MainState, OutputChannel>(|| {
        MainState::new(None, ToolbarState::new())
    });
    let mut cavas_radio = use_radio(OutputChannel::Canvas);

    spawn(async move {
        let mut ticker = consume_root_context::<RenderingTicker>();

        loop {
            if let Ok(output) = output_rx.try_recv() {
                let mut main_state = cavas_radio.write();
                match output {
                    OutputSignal::Frame(image) => match main_state.canvas.as_mut() {
                        Some(canvas) => canvas.frame = Some(image),
                        None => (),
                    },
                    OutputSignal::Viewport(viewport) => match main_state.canvas.as_mut() {
                        Some(canvas_state) => {
                            canvas_state.look_at = viewport.look_at;
                            canvas_state.right = viewport.right;
                            canvas_state.fov = viewport.fov;
                        }
                        _ => {
                            let mut canvas: CanvasState = Default::default();
                            canvas.look_at = viewport.look_at;
                            canvas.right = viewport.right;
                            canvas.fov = viewport.fov;
                            main_state.canvas = Some(canvas);
                        }
                    },
                }
            };

            ticker.tick().await;
        }
    });

    rect()
        .center()
        .expanded()
        .child(main_window(input_tx_state))
}
