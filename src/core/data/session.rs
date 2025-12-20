use crate::core::data::project::Project;

pub struct Session {
    pub project: Option<Project>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            project: None,
        }
    }
}