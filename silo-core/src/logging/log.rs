use pretty_env_logger;

pub struct Logger {}

impl Logger {
    /// Creates and returns a new instance of the Logger.
    pub fn new() -> Self {
        Self {}
    }

    /// Initializes the logger.
    pub fn init(&self) -> Result<(), std::io::Error> {
        // TODO: replace with a custom logging solution.
        // For now, I'm just using pretty-env-logger.
        pretty_env_logger::init();

        Ok(())
    }
}
