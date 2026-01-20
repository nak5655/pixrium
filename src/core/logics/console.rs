use crate::core::data::Session;
use crate::core::inputs::Input;
use crate::core::logics::commands::*;
use crate::core::logics::tools::{BrushTool, EventHandling, PanTool, Tool, Tools, ZoomTool};
use crate::core::services::Services;
use std::collections::HashMap;

pub struct Console<S: Services> {
    pub session: Session,
    pub services: S,
    pub tools: HashMap<Tools, Box<dyn Tool<S>>>,
    active_tools: Vec<Tools>,
}

impl<S: Services> Console<S> {
    pub fn new(session: Session, services: S) -> Self {
        let mut tools: HashMap<Tools, Box<dyn Tool<S>>> = HashMap::new();
        tools.insert(Tools::Pan, Box::new(PanTool::new()));
        tools.insert(Tools::Brush, Box::new(BrushTool::new()));
        tools.insert(Tools::Zoom, Box::new(ZoomTool::new()));

        Self {
            session,
            services,
            tools,
            active_tools: vec![Tools::Brush, Tools::Zoom],
        }
    }

    pub fn execute(&mut self, command: &dyn Command<S>) {
        command.execute(&self.services, &mut self.session)
    }

    pub fn input(&mut self, input: &Input) {
        for tools in &self.active_tools {
            let tool = self.tools.get_mut(tools).expect("invalid tool specified");
            match input {
                Input::Pointer(pointer_input) => {
                    match tool.pointer_input(pointer_input, &mut self.session, &mut self.services) {
                        EventHandling::Captured => return,
                        _ => (),
                    }
                }
                Input::Keyboard(keyboard_input) => {
                    match tool.keyboard_input(keyboard_input, &mut self.session, &mut self.services)
                    {
                        EventHandling::Captured => return,
                        _ => (),
                    }
                }
                _ => return,
            }
        }
    }
}
