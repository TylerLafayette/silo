use async_trait::async_trait;
use oxidizer::*;
use silo_core::models;

use crate::config::DatabaseConfig;
use crate::connection::*;
use crate::db_utils::postgres_conn_str;
use crate::errors::*;
use crate::models as db_models;

/// A trait of methods implemented by the Service.
#[async_trait]
pub trait Service: Sync + Send {
    /// Inserts a SubjectTrait into the database.
    async fn insert_subject_trait(
        &self,
        subject_trait: &models::SubjectTrait,
    ) -> Result<i16, DatabaseError>;
    /// Inserts a Subject into the database.
    async fn insert_subject(&self, subject: &models::Subject) -> Result<i16, DatabaseError>;
    /// Inserts a Group into the database.
    async fn insert_group(&self, group: &models::Group) -> Result<i16, DatabaseError>;
}

/// An implementation of the service itself.
pub struct ServiceImpl {
    conn: Box<Connection>,
}

impl ServiceImpl {
    /// Creates and returns a new ServiceImpl with the provided DatabaseConfig.
    pub fn new(conn: Box<Connection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Service for ServiceImpl {
    async fn insert_subject_trait(
        &self,
        subject_trait: &models::SubjectTrait,
    ) -> Result<i16, DatabaseError> {
        let mut st = db_models::SubjectTrait::from(subject_trait);
        match st.save(&self.conn.db).await {
            Ok(_) => Ok(st.id),
            Err(_) => Err(DatabaseError("failed to insert".into())),
        }
    }
    async fn insert_subject(&self, subject: &models::Subject) -> Result<i16, DatabaseError> {
        let mut s = db_models::Subject::from(subject);
        match s.save(&self.conn.db).await {
            Ok(_) => Ok(s.id),
            Err(_) => Err(DatabaseError("failed to insert".into())),
        }
    }
    async fn insert_group(&self, group: &models::Group) -> Result<i16, DatabaseError> {
        let mut g = db_models::Group::from(group);
        match g.save(&self.conn.db).await {
            Ok(_) => Ok(g.id),
            Err(_) => Err(DatabaseError("failed to insert".into())),
        }
    }
}
