use serde::Serialize;

/// Represents a runnable job.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResult {
    /// The job result's unique ID.
    pub id: i32,

    /// The idea of the job the result is attached to.
    pub job_id: i32,

    /// Whether or not there was an error.
    pub error: bool,

    /// The result as a single line.
    pub single_line: String,
}
