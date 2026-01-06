use crate::core::data::project::Project;
use crate::core::logics::tools::{Tool, Tools};
use std::collections::HashMap;
use crate::core::inputs::Input;

pub struct Session {
    pub project: Option<Project>,
    pub selected_layer_index: Option<usize>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
            selected_layer_index: None,
        }
    }
}
