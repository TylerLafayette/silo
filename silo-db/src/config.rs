/// DatabaseConfig contains database connection credentials and other configuration.
#[derive(Debug)]
pub struct DatabaseConfig {
    /// The username for the database.
    pub databaseUsername: String,
    /// The password for the database.
    pub databasePassword: String,
    /// The host of the database.
    pub databaseHost: String,
    /// The port of the database.
    pub databasePort: String,
    /// The name of the database.
    pub databaseName: String,
}
