use crate::core::data::{Project, Session};
use crate::core::input::commands::*;
use crate::core::input::InputSignal;
use crate::core::logics::tools::{BrushTool, EventHandling, PanTool, Tool, Tools, ZoomTool};
use crate::core::output::OutputSignal;
use crate::core::services::Services;
use std::collections::HashMap;
use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

pub struct Console<S: Services> {
    output_tx: Sender<OutputSignal>,
    pub session: Option<Session>,
    pub services: S,
    pub tools: HashMap<Tools, Box<dyn Tool<S>>>,
    fallback_tools: Vec<Tools>,
    pub active_tool: Tools,
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
            active_tool: Tools::Brush,
        }
    }

    pub fn open_session(&mut self, project: Project) -> &mut Session {
        let session = Session::new(project, self.output_tx.clone());
        self.session = Some(session);
        self.session.as_mut().unwrap()
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

        for tools in once(&self.active_tool).chain(&self.fallback_tools) {
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
