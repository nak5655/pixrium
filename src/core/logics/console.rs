use crate::core::data::Session;
use crate::core::inputs::Input;
use crate::core::logics::commands::*;
use crate::core::logics::tools::{BrushTool, EventHandling, PanTool, Tool, Tools, ZoomTool};
use crate::core::services::Services;
use std::collections::HashMap;
use std::iter::once;

pub struct Console<S: Services> {
    pub session: Option<Session>,
    pub services: S,
    pub tools: HashMap<Tools, Box<dyn Tool<S>>>,
    fallback_tools: Vec<Tools>,
    pub active_tool: Tools,
}

impl<S: Services> Console<S> {
    pub fn new(services: S) -> Self {
        let mut tools: HashMap<Tools, Box<dyn Tool<S>>> = HashMap::new();
        tools.insert(Tools::Pan, Box::new(PanTool::new()));
        tools.insert(Tools::Brush, Box::new(BrushTool::new()));
        tools.insert(Tools::Zoom, Box::new(ZoomTool::new()));

        Self {
            session: None,
            services,
            tools,
            fallback_tools: vec![Tools::Pan, Tools::Zoom],
            active_tool: Tools::Brush,
        }
    }

    pub fn execute(&mut self, command: &impl Command<S>) {
        command.execute(self);
    }

    pub fn input(&mut self, input: &Input) {
        let session = match self.session.as_mut() {
            Some(session) => session,
            _ => return,
        };

        for tools in once(&self.active_tool).chain(&self.fallback_tools) {
            let tool = self.tools.get_mut(tools).expect("invalid tool specified");
            match input {
                Input::Pointer(pointer_input) => {
                    match tool.pointer_input(pointer_input, session, &mut self.services) {
                        EventHandling::Captured => return,
                        _ => (),
                    }
                }
                Input::Keyboard(keyboard_input) => {
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
