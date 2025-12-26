use crate::core::data::Session;
use crate::core::services::Services;

mod open_file_command;
mod show_version_command;
mod layer_dock_command;

pub use layer_dock_command::*;
pub use open_file_command::*;
pub use show_version_command::*;

pub trait Command<S: Services> {
    fn execute(&self, services: &S, session: &mut Session);
}