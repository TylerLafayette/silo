#![deny(missing_docs)]

//! The CLI entrypoint for silo.
//! silo initializes the silo_core service and runs it.

use log::{error, info};
use silo_core::service::Service;

/// Initializes the silo_core Service and runs it.
fn main() {
    let service = Service::new();
    match service.run() {
        Ok(_) => info!("Service exited gracefully"),
        Err(e) => error!("Error starting service: {}", e),
    };
}
