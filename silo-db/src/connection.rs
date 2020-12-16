use oxidizer::*;
use tokio;

use crate::config::*;
use crate::db_utils::postgres_conn_str;
use crate::errors::*;
use crate::migrations;

/// Connection represents a connection to a database.
pub struct Connection {
    /// The database connection object.
    pub db: DB,
}

impl Connection {
    /// Attempts to connect to a database and returns a Connection on success.
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, ConnectionError> {
        let uri = postgres_conn_str(config);
        match DB::connect(&uri, 50, None).await {
            Ok(db) => Ok(Self { db }),
            Err(_) => Err(ConnectionError("error connecting to database".into())),
        }
    }
    /// Attempts to run migrations.
    pub async fn migrate(&self) -> Result<(), DatabaseError> {
        let runner = migrations::runner();

        match self.db.migrate(runner).await {
            Ok(_) => Ok(()),
            Err(_) => Err(DatabaseError("error running migrations".into())),
        }
    }
}
