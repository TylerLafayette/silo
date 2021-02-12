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
    ) -> Result<i32, DatabaseError>;
    /// Inserts a Subject into the database.
    async fn insert_subject(&self, subject: &models::Subject) -> Result<i32, DatabaseError>;
    /// Inserts a Group into the database.
    async fn insert_group(&self, group: &models::Group) -> Result<i32, DatabaseError>;
    /// Adds a SubjectTrait to a Subject by the Subject and Trait IDs.
    async fn insert_subject_subject_trait(
        &self,
        subject_id: i32,
        subject_trait_id: i32,
    ) -> Result<i32, DatabaseError>;
    /// Finds all traits.
    async fn get_traits(&self) -> Result<Vec<models::SubjectTrait>, DatabaseError>;
    /// Finds all groups.
    async fn get_groups(&self) -> Result<Vec<models::Group>, DatabaseError>;
    /// Finds all subject traits for a subject by ID.
    async fn find_subject_trats_by_subject_id(
        &self,
        id: i32,
    ) -> Result<Vec<models::SubjectTrait>, DatabaseError>;
    /// Finds a single Group by ID.
    async fn find_group_by_id(&self, id: i32) -> Result<Option<models::Group>, DatabaseError>;
    /// Finds a single Subject by ID.
    async fn find_subject_by_id(&self, id: i32) -> Result<Option<models::Subject>, DatabaseError>;
    /// Finds all subjects in a single Group by its ID.
    async fn find_subjects_by_group_id(
        &self,
        id: i32,
    ) -> Result<Vec<models::Subject>, DatabaseError>;
    /// Finds a single SubjectTrait by ID.
    async fn find_subject_trait_by_id(
        &self,
        id: i32,
    ) -> Result<Option<models::SubjectTrait>, DatabaseError>;
    /// Finds a single SubjectTrait by trait name.
    async fn find_subject_trait_by_name(
        &self,
        trait_name: &str,
    ) -> Result<Option<models::SubjectTrait>, DatabaseError>;
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
    ) -> Result<i32, DatabaseError> {
        let mut st = db_models::SubjectTrait::from(subject_trait);
        match st.save(&self.conn.db).await {
            Ok(_) => Ok(st.id),
            Err(e) => Err(DatabaseError(format!("{:?}", e))),
        }
    }
    async fn insert_subject(&self, subject: &models::Subject) -> Result<i32, DatabaseError> {
        let mut s = db_models::Subject::from(subject);
        match s.save(&self.conn.db).await {
            Ok(_) => Ok(s.id),
            Err(e) => Err(DatabaseError(format!("{:?}", e))),
        }
    }
    async fn insert_group(&self, group: &models::Group) -> Result<i32, DatabaseError> {
        let mut g = db_models::Group::from(group);
        match g.save(&self.conn.db).await {
            Ok(_) => Ok(g.id),
            Err(e) => Err(DatabaseError(format!("{:?}", e))),
        }
    }
    async fn insert_subject_subject_trait(
        &self,
        subject_id: i32,
        subject_trait_id: i32,
    ) -> Result<i32, DatabaseError> {
        let mut sst = db_models::SubjectSubjectTrait {
            id: 0,
            subject_id,
            subject_trait_id,
        };
        match sst.save(&self.conn.db).await {
            Ok(_) => Ok(sst.id),
            Err(e) => Err(DatabaseError(format!("{:?}", e))),
        }
    }
    async fn get_traits(&self) -> Result<Vec<models::SubjectTrait>, DatabaseError> {
        let t = db_models::SubjectTrait::find(&self.conn.db, "id > 0", &[])
            .await
            .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?;

        Ok(t.iter()
            .map(|x| models::SubjectTrait {
                id: x.id,
                parent_id: x.parent_id,
                trait_name: x.trait_name.clone(),
            })
            .collect())
    }
    async fn get_groups(&self) -> Result<Vec<models::Group>, DatabaseError> {
        let g = db_models::Group::find(&self.conn.db, "id > 0", &[])
            .await
            .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?;

        Ok(g.iter().map(|x| models::Group { id: x.id }).collect())
    }
    async fn find_subject_trats_by_subject_id(
        &self,
        id: i32,
    ) -> Result<Vec<models::SubjectTrait>, DatabaseError> {
        let st = db_models::SubjectSubjectTrait::find(&self.conn.db, "subject_id = $1", &[&id])
            .await
            .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?;

        let mut sub_traits = vec![];
        for s in st {
            let sub_trait =
                db_models::SubjectTrait::first(&self.conn.db, "id = $1", &[&s.subject_trait_id])
                    .await
                    .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?;

            match sub_trait {
                Some(s) => {
                    sub_traits.push(models::SubjectTrait {
                        id: s.id,
                        parent_id: s.parent_id,
                        trait_name: s.trait_name,
                    });
                }
                None => (),
            };
        }

        Ok(sub_traits)
    }
    async fn find_group_by_id(&self, id: i32) -> Result<Option<models::Group>, DatabaseError> {
        let g = match db_models::Group::first(&self.conn.db, "id = $1", &[&id]).await {
            Ok(Some(g)) => g,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DatabaseError(format!("{:?}", e))),
        };

        Ok(Some(models::Group { id: g.id }))
    }
    async fn find_subject_by_id(&self, id: i32) -> Result<Option<models::Subject>, DatabaseError> {
        let s = match db_models::Subject::first(&self.conn.db, "id = $1", &[&id]).await {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DatabaseError(format!("{:?}", e))),
        };

        Ok(Some(models::Subject {
            id: s.id,
            group_id: s.group_id,
            age: s.age,
            length_of_stay: s.length_of_stay,
        }))
    }
    async fn find_subjects_by_group_id(
        &self,
        id: i32,
    ) -> Result<Vec<models::Subject>, DatabaseError> {
        let subjects = db_models::Subject::find(&self.conn.db, "group_id = $1", &[&id])
            .await
            .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?
            .iter()
            .map(|s| models::Subject {
                id: s.id,
                group_id: s.group_id,
                age: s.age,
                length_of_stay: s.length_of_stay,
            })
            .collect();

        Ok(subjects)
    }
    async fn find_subject_trait_by_id(
        &self,
        id: i32,
    ) -> Result<Option<models::SubjectTrait>, DatabaseError> {
        let st = match db_models::SubjectTrait::first(&self.conn.db, "id = $1", &[&id]).await {
            Ok(Some(st)) => st,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DatabaseError(format!("{:?}", e))),
        };

        Ok(Some(models::SubjectTrait {
            id: st.id,
            parent_id: st.parent_id,
            trait_name: st.trait_name,
        }))
    }
    async fn find_subject_trait_by_name(
        &self,
        trait_name: &str,
    ) -> Result<Option<models::SubjectTrait>, DatabaseError> {
        let st = db_models::SubjectTrait::first(&self.conn.db, "trait_name = $1", &[&trait_name])
            .await
            .or_else(|e| Err(DatabaseError(format!("{:?}", e))))?;

        let s = match st {
            Some(s) => s,
            None => return Ok(None),
        };

        Ok(Some(models::SubjectTrait {
            id: s.id,
            parent_id: s.parent_id,
            trait_name: s.trait_name,
        }))
    }
}
