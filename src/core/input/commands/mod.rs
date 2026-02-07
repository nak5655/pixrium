use crate::core::services::Services;

mod layer_dock_command;
mod open_file_command;
mod show_version_command;

use crate::core::logics::Console;
pub use layer_dock_command::*;
pub use open_file_command::*;
pub use show_version_command::*;

pub trait Command<S: Services>: Send + Sync {
    fn execute(&self, console: &mut Console<S>);
}
