use crate::core::data::config::{CommandId, Config};
use crate::core::input::Key;

pub struct ConfigService {
    config: Config,
}

impl ConfigService {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn get_key_binding(&self, command_id: CommandId) -> Key {
        match self.config.key_bindings.get(&command_id) {
            Some(key) => key.clone(),
            _ => Key::Unidentified,
        }
    }
}
