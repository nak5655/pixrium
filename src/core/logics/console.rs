use crate::core::data::Session;
use crate::core::logics::commands::*;
use crate::core::services::Services;
use std::collections::HashMap;

pub struct Console<S: Services> {
    pub(crate) session: Session,
    services: S,
    commands: HashMap<Commands, Box<dyn Command<S>>>,
}

impl<S: Services> Console<S> {
    pub fn new(session: Session, services: S) -> Self {
        let mut console = Self {
            session,
            services,
            commands: HashMap::new(),
        };

        console.register(Commands::OpenFile, Box::new(OpenFileCommand {}));
        console.register(Commands::ShowVersion, Box::new(ShowVersionCommand {}));

        console
    }

    pub fn register(&mut self, key: Commands, command: Box<dyn Command<S>>) {
        self.commands.insert(key, command);
    }

    pub fn run(&mut self, key: Commands) {
        self.commands.get(&key).map(|command| {
            command.run(&self.services, &mut self.session)
        }).unwrap_or_else(|| {
            println!("no command")
        });
    }
}