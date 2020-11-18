use crate::logging::log::Logger;
use actix::prelude::*;
use log::info;

/// Represents the silo core service.
pub struct Service {
    logger: Logger,
}

/// Represents the implementation of the Service.
impl Service {
    /// Create and return a new Service.
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }

    /// Start the actix system and run the Service.
    pub fn run(&self) -> Result<(), &str> {
        self.logger.init().or(Err("could not start logger"))?;

        let system = System::new("silo");
        info!("actix system running");
        system.run().or(Err("error starting the actix system"))
    }
}
