use crate::core::data::Session;
use crate::core::logics::commands::*;
use crate::core::services::Services;

pub struct Console<S: Services> {
    pub(crate) session: Session,
    services: S,
}

impl<S: Services> Console<S> {
    pub fn new(session: Session, services: S) -> Self {
        Self {
            session,
            services,
        }
    }

    pub fn execute(&mut self, command: &dyn Command<S>) {
        command.execute(&self.services, &mut self.session)
    }
}