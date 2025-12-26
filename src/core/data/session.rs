use crate::core::data::project::Project;
use crate::core::logics::tools::{Tool, Tools};
use std::collections::HashMap;

pub struct Session {
    pub project: Option<Project>,
    pub selected_layer_index: Option<usize>,
    pub tools: HashMap<Tools, Box<dyn Tool>>,
    pub selected_tool: Tools,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
            selected_layer_index: None,
            tools: HashMap::new(),
            selected_tool: Tools::Pan,
        }
    }
}
