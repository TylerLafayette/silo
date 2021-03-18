use serde::Serialize;
use serde_repr::*;

/// Represents the status of a single job.
#[derive(Debug, Serialize)]
#[repr(u8)]
pub enum JobStatus {
    /// Represents an inactive job.
    Inactive = 0,

    /// Represents a running job.
    Active = 1,
}

/// Represents a runnable job.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// The job's unique ID.
    pub id: i32,

    /// The raw code of the script.
    ///
    /// TODO: Refactor into a better storage method.
    pub code_raw: String,

    /// The job's status.
    pub status: JobStatus,
}

impl From<i16> for JobStatus {
    fn from(status: i16) -> Self {
        match status {
            0 => JobStatus::Inactive,
            1 => JobStatus::Active,
            _ => JobStatus::Inactive,
        }
    }
}
