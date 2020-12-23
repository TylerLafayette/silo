#![deny(missing_docs)]

//! The CLI entrypoint for silo.
//! silo initializes the silo_core service and runs it.

use actix::prelude::*;
use log::{error, info};
use tokio::prelude::*;

use silo_db::actor::*;
use silo_db::config::DatabaseConfig;
use silo_db::connection::Connection as DbConnection;
use silo_db::service::{Service as DbServiceTrait, ServiceImpl};

use silo_core::models::SubjectTrait;
use silo_core::service::Service;

/// Initializes the silo_core Service and runs it.
#[tokio::main]
async fn main() -> Result<(), String> {
    let db_config = DatabaseConfig {
        database_host: String::from("localhost"),
        database_port: String::from("5432"),
        database_username: String::from("postgres"),
        database_password: String::from("postgres"),
        database_name: String::from("silo"),
    };

    let conn = DbConnection::connect(&db_config).await.unwrap();
    let db_service = ServiceImpl::new(&conn);

    conn.migrate()
        .await
        .or_else(|_| Err("failed to migrate db"))?;

    // Start the Actix system.
    let service = Service::new();
    match service.run() {
        Ok(_) => info!("Service exited gracefully"),
        Err(e) => error!("Error starting service: {}", e),
    };

    // let addr = DbActor::new(&db_service).start();

    Ok(())
}
