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
pub trait Service {
    /// Inserts a SubjectTrait into the database.
    async fn insert_subject_trait(
        &self,
        subject_trait: &models::SubjectTrait,
    ) -> Result<i32, DatabaseError>;
}

/// An implementation of the service itself.
pub struct ServiceImpl<'a> {
    conn: &'a Connection,
}

impl<'a> ServiceImpl<'a> {
    /// Creates and returns a new ServiceImpl with the provided DatabaseConfig.
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> Service for ServiceImpl<'a> {
    async fn insert_subject_trait(
        &self,
        subject_trait: &models::SubjectTrait,
    ) -> Result<i32, DatabaseError> {
        let mut st = db_models::SubjectTrait::from(subject_trait);
        match st.save(&self.conn.db).await {
            Ok(_) => Ok(st.id),
            Err(_) => Err(DatabaseError("failed to insert".into())),
        }
    }
}
