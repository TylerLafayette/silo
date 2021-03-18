use oxidizer::*;

use silo_core::models;

/// Contains a single trait which can be applied to subjects.
#[derive(Entity, Default)]
pub struct SubjectTrait {
    /// The ID of the entity.
    #[primary_key]
    pub id: i32,

    /// The ID of the parent, if one exists.
    #[relation(model = "SubjectTrait", key = "id")]
    pub parent_id: i32,

    /// The name of the trait.
    #[indexed]
    pub trait_name: String,
}

impl From<&models::SubjectTrait> for SubjectTrait {
    fn from(item: &models::SubjectTrait) -> Self {
        Self {
            id: item.id,
            parent_id: item.parent_id,
            trait_name: item.trait_name.clone(),
        }
    }
}

/// Contains information about a subject which can be connected with traits.
#[derive(Entity, Default)]
pub struct Subject {
    #[primary_key]
    pub id: i32,

    #[relation(model = "Group", key = "id")]
    pub group_id: i32,

    pub age: i16,
    pub length_of_stay: i16,
}

impl From<&models::Subject> for Subject {
    fn from(item: &models::Subject) -> Self {
        Self {
            id: item.id,
            group_id: item.group_id,
            age: item.age,
            length_of_stay: item.length_of_stay,
        }
    }
}

/// Group groups subjects for analysis.
#[derive(Entity, Default)]
#[entity(table_name = "subject_group")]
pub struct Group {
    #[primary_key]
    pub id: i32,
}

impl From<&models::Group> for Group {
    fn from(item: &models::Group) -> Self {
        Self { id: item.id }
    }
}

/// SubjectSubjectTrait acts as a join table for Subjects to
/// SubjecTraits.
#[derive(Entity, Default)]
pub struct SubjectSubjectTrait {
    #[primary_key]
    pub id: i32,

    #[relation(model = "Subject", key = "id")]
    pub subject_id: i32,

    #[relation(model = "SubjectTrait", key = "id")]
    pub subject_trait_id: i32,
}

/// Represents a runnable job.
#[derive(Entity)]
pub struct Job {
    /// The job's unique ID.
    #[primary_key]
    pub id: i32,

    /// The job's raw code.
    pub code_raw: String,

    /// The job's status.
    pub status: i16,
}

impl Into<models::Job> for &Job {
    fn into(self) -> models::Job {
        models::Job {
            id: self.id,
            code_raw: self.code_raw.clone(),
            status: models::JobStatus::from(self.status),
        }
    }
}

/// Represents the result of running a Job.
#[derive(Entity)]
pub struct JobResult {
    /// The job result's unique ID.
    #[primary_key]
    pub id: i32,

    /// The idea of the job the result is attached to.
    #[relation(model = "Job", key = "id")]
    pub job_id: i32,

    /// The job's raw code.
    pub error: bool,

    /// The result as a single line.
    pub single_line: String,
}

impl Into<models::JobResult> for &JobResult {
    fn into(self) -> models::JobResult {
        models::JobResult {
            id: self.id,
            job_id: self.job_id,
            error: self.error,
            single_line: self.single_line.clone(),
        }
    }
}
