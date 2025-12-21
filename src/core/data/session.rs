use crate::core::data::project::Project;

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