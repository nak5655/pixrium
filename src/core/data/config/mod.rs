mod command_id;

pub use self::command_id::*;

use std::collections::HashMap;
use crate::core::inputs::Key;

pub struct Config {
    pub key_bindings: HashMap<CommandId, Key>
}

impl Default for Config {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert(CommandId::ZoomIn, Key::Character("[".to_string()));
        key_bindings.insert(CommandId::ZoomOut, Key::Character("]".to_string()));

        Self {
            key_bindings
        }
    }
}