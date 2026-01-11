use crate::core::data::project::Project;
use crate::core::data::Layer;

pub struct Session {
    pub project: Option<Project>,
    pub selected_layer_index: usize,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
            selected_layer_index: 0,
        }
    }

    pub fn layers(&self) -> Vec<&Layer> {
        match &self.project {
            Some(project) => project.layers.iter().collect(),
            None => return vec![],
        }
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        let project = match &self.project {
            Some(project) => project,
            None => return None,
        };

        if 0 <= self.selected_layer_index && self.selected_layer_index < project.layers.len() {
            Some(&project.layers[self.selected_layer_index])
        } else {
            None
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut Layer> {
        let mut project = match &mut self.project {
            Some(project) => project,
            None => return None,
        };

        if 0 <= self.selected_layer_index && self.selected_layer_index < project.layers.len() {
            Some(&mut project.layers[self.selected_layer_index])
        } else {
            None
        }
    }
}
