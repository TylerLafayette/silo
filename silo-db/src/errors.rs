use std::fmt::{Display, Formatter, Result};

/// ConnectionError is returned when there is an issue connecting to the database.
#[derive(Debug, Clone)]
pub struct ConnectionError(pub String);

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

/// DatabaseError is returned when a database operation fails.
#[derive(Debug, Clone)]
pub struct DatabaseError(pub String);

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
