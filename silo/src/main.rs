#![deny(missing_docs)]

//! The CLI entrypoint for silo.
//! silo initializes the silo_core service and runs it.

use log::{error, info};
use tokio::prelude::*;

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

    conn.migrate().await;

    let service = ServiceImpl::new(&conn);

    let test_trait = SubjectTrait {
        id: 0,
        parent_id: 0,
        trait_name: "test".into(),
    };

    match service.insert_subject_trait(&test_trait).await {
        Ok(id) => println!("Success, ID: {}", id),
        Err(e) => println!("Failure: {}", e),
    };

    // let service = Service::new();
    // match service.run() {
    //     Ok(_) => info!("Service exited gracefully"),
    //     Err(e) => error!("Error starting service: {}", e),
    // };

    Ok(())
}
