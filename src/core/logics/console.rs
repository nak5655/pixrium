use std::collections::HashMap;
use crate::core::data::{Session};
use crate::core::inputs::Input;
use crate::core::logics::commands::*;
use crate::core::logics::tools::{Tool, Tools};
use crate::core::services::Services;

pub struct Console<S: Services> {
    pub session: Session,
    pub services: S,
    pub tools: HashMap<Tools, Box<dyn Tool<S>>>,
    current_tool: Tools,
}

impl<S: Services> Console<S> {
    pub fn new(session: Session, services: S, tools: HashMap<Tools, Box<dyn Tool<S>>>) -> Self {
        Self {
            session,
            services,
            tools,
            current_tool: Tools::Pan,
        }
    }

    pub fn execute(&mut self, command: &dyn Command<S>) {
        command.execute(&self.services, &mut self.session)
    }

    pub fn input(&mut self, input: &Input) {
        let tool = self.tools.get_mut(&self.current_tool).unwrap();

        let project = match &mut self.session.project {
            Some(project) => project,
            None => return,
        };

        match input {
            Input::PointerInput(pointer_input) => {
                let _ = tool.pointer_input(pointer_input, project, &mut self.services);
            },
            _ => return,
        }
    }
}
