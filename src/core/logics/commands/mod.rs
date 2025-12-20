use crate::core::data::Session;
use crate::core::services::Services;

mod open_file_command;
mod show_version_command;

pub use open_file_command::*;

pub use show_version_command::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Commands {
    OpenFile,
    ShowVersion,
}

pub trait Command<S: Services> {
    fn run(&self, services: &S, session: &mut Session);
}