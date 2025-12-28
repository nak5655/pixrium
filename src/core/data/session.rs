use crate::core::data::project::Project;
use crate::core::logics::tools::{Tool, Tools};
use std::collections::HashMap;
use crate::core::inputs::Input;

pub struct Session {
    pub project: Option<Project>,
    pub selected_layer_index: Option<usize>,
    pub tools: HashMap<Tools, Box<dyn Tool>>,
    current_tool: Tools,
}

impl Session {
    pub fn new(tools: HashMap<Tools, Box<dyn Tool>>) -> Self {
        Self {
            project: None,
            selected_layer_index: None,
            tools,
            current_tool: Tools::Pan,
        }
    }

    pub fn input(&mut self, input: &Input) {
        let tool = self.tools.get_mut(&self.current_tool).unwrap();

        let project = match &mut self.project {
            Some(project) => project,
            None => return,
        };

        match input {
            Input::PointerInput(pointer_input) => tool.pointer_input(project, pointer_input),
        }
    }
}
