use crate::config::DatabaseConfig;

/// Generates a postgres connection string from a DatabaseConfig pointer.
pub fn postgres_conn_str(config: &DatabaseConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_username,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name,
    )
}
