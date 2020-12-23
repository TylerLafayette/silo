use crate::errors::*;
use crate::models::*;
use crate::service::Service;
use actix::prelude::*;
use async_trait::async_trait;
use silo_core::models;
use tokio::task;

/// Actix actor for the DB.
pub struct DbActor {
    service: &'static dyn Service,
}

impl Actor for DbActor {
    type Context = Context<Self>;
}

impl DbActor {
    /// Creates and returns a DbActor from a Service.
    pub fn new(service: &'static dyn Service) -> Self {
        Self { service }
    }
}

/// Inserts a Subject into the DB.
#[derive(Message)]
#[rtype(result = "Result<i16, DatabaseError>")]
pub struct InsertSubject(&'static models::Subject);

impl Handler<InsertSubject> for DbActor {
    type Result = ResponseFuture<Result<i16, DatabaseError>>;

    fn handle(&mut self, msg: InsertSubject, ctx: &mut Self::Context) -> Self::Result {
        let query = self.service.insert_subject(msg.0);

        Box::pin(async move { query.await })
    }
}

/// Inserts a SubjectTrait into the DB.
#[derive(Message)]
#[rtype(result = "Result<i16, DatabaseError>")]
pub struct InsertSubjectTrait(&'static models::SubjectTrait);

impl Handler<InsertSubjectTrait> for DbActor {
    type Result = ResponseFuture<Result<i16, DatabaseError>>;

    fn handle(&mut self, msg: InsertSubjectTrait, ctx: &mut Self::Context) -> Self::Result {
        let query = self.service.insert_subject_trait(msg.0);

        Box::pin(async move { query.await })
    }
}

/// Inserts a Group into the DB.
#[derive(Message)]
#[rtype(result = "Result<i16, DatabaseError>")]
pub struct InsertGroup(&'static models::Group);

impl Handler<InsertGroup> for DbActor {
    type Result = ResponseFuture<Result<i16, DatabaseError>>;

    fn handle(&mut self, msg: InsertGroup, ctx: &mut Self::Context) -> Self::Result {
        let query = self.service.insert_group(msg.0);

        Box::pin(async move { query.await })
    }
}
