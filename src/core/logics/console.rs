use crate::core::data::{Layer, LayerState, Project, Session};
use crate::core::input::commands::*;
use crate::core::input::InputSignal;
use crate::core::logics::tools::{BrushTool, EventHandling, PanTool, Tool, Tools, ZoomTool};
use crate::core::output::OutputSignal;
use crate::core::services::Services;
use skia_safe::Rect;
use std::collections::HashMap;
use std::iter::once;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

pub struct Console<S: Services> {
    output_tx: Sender<OutputSignal>,
    pub session: Option<Session>,
    pub services: S,
    pub tools: HashMap<Tools, Box<dyn Tool<S>>>,
    fallback_tools: Vec<Tools>,
}

impl<S: Services> Console<S> {
    pub fn new(services: S, output_tx: Sender<OutputSignal>) -> Self {
        let mut tools: HashMap<Tools, Box<dyn Tool<S>>> = HashMap::new();
        tools.insert(Tools::Pan, Box::new(PanTool::new()));
        tools.insert(Tools::Brush, Box::new(BrushTool::new()));
        tools.insert(Tools::Zoom, Box::new(ZoomTool::new()));

        Self {
            output_tx,
            session: None,
            services,
            tools,
            fallback_tools: vec![Tools::Pan, Tools::Zoom],
        }
    }

    pub fn open_session_from_image(&mut self, image_path: PathBuf) -> Result<(), String> {
        Layer::load(image_path).map(|layer| {
            let mut project = Project::new(
                layer.bitmap.width() as usize,
                layer.bitmap.height() as usize,
            );
            project.layers.push(layer);
            _ = self.output_tx.send(OutputSignal::Layers(Some(
                project
                    .layers
                    .iter()
                    .map(|layer| LayerState {
                        name: layer.name.clone(),
                        opacity: layer.opacity.clone(),
                    })
                    .collect(),
            )));

            let mut session = Session::new(project.width, project.height, self.output_tx.clone());
            session.project = project;
            session.state.selected_layer_index = session.project.layers.len() - 1;
            session.update_frame(Rect::new(
                0.0,
                0.0,
                session.project.width as f32,
                session.project.height as f32,
            ));

            self.session = Some(session);
        })
    }

    pub fn update(&mut self, input_rx: &Receiver<InputSignal<S>>) {
        if let Ok(input) = input_rx.try_recv() {
            self.input(&input)
        }
    }

    pub fn execute(&mut self, command: &dyn Command<S>) {
        command.execute(self);
    }

    pub fn input(&mut self, input: &InputSignal<S>) {
        match input {
            InputSignal::Command(command) => {
                return command.execute(self);
            }
            _ => (),
        }

        let session = match self.session.as_mut() {
            Some(session) => session,
            _ => return,
        };

        match input {
            InputSignal::ViewportResized(size) => {
                session.state.viewport.size = size.clone();
            }
            InputSignal::ChooseTool(tool) => {
                session.state.active_tool = *tool;
                _ = self.output_tx.send(OutputSignal::ActiveTool(*tool));
            }
            _ => (),
        }
        for tools in once(&session.state.active_tool.clone()).chain(&self.fallback_tools) {
            let tool = self.tools.get_mut(tools).expect("invalid tool specified");
            match input {
                InputSignal::Pointer(pointer_input) => {
                    match tool.pointer_input(pointer_input, session, &mut self.services) {
                        EventHandling::Captured => return,
                        _ => (),
                    }
                }
                InputSignal::Keyboard(keyboard_input) => {
                    match tool.keyboard_input(keyboard_input, session, &mut self.services) {
                        EventHandling::Captured => return,
                        _ => (),
                    }
                }
                _ => return,
            }
        }
    }
}
