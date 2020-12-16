use crate::config::DatabaseConfig;

/// Generates a postgres connection string from a DatabaseConfig pointer.
pub fn postgres_conn_str(config: &DatabaseConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.databaseUsername,
        config.databasePassword,
        config.databaseHost,
        config.databasePort,
        config.databaseName
    )
}
