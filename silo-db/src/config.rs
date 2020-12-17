/// DatabaseConfig contains database connection credentials and other configuration.
#[derive(Debug)]
pub struct DatabaseConfig {
    /// The username for the database.
    pub database_username: String,
    /// The password for the database.
    pub database_password: String,
    /// The host of the database.
    pub database_host: String,
    /// The port of the database.
    pub database_port: String,
    /// The name of the database.
    pub database_name: String,
}
