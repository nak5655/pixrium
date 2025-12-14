use freya::prelude::*;

mod gui;
mod core;

use crate::gui::windows::main::main_window::MainWindow;

fn main() {
    launch(MainWindow);
}
