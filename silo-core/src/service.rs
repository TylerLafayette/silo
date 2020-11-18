use actix::prelude::*;

/// Represents the silo core service.
pub struct Service {}

/// Represents the implementation of the Service.
impl Service {
    /// Create and return a new Service.
    pub fn new() -> Self {
        Self {}
    }

    /// Start the actix system and run the Service.
    pub fn run(&self) -> Result<(), &str> {
        let system = System::new("silo");
        system.run().or(Err("machine broke"))
    }
}
