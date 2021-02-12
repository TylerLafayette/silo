use serde::Serialize;

/// Contains information about a single subject for analysis.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// The subject's unique ID.
    pub id: i32,
    /// The ID of the group the subject belongs to.
    pub group_id: i32,
    /// The subject's age.
    pub age: i16,
    /// The subject's length of stay.
    pub length_of_stay: i16,
}
