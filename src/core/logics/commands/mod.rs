use crate::core::data::Session;
use crate::core::services::Services;

pub mod open_file_command;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Commands {
    OpenFile,
}

pub trait Command<S: Services> {
    fn run(&self, services: &S, session: &mut Session);
}