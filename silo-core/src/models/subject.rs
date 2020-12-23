/// Contains information about a single subject for analysis.
#[derive(Debug, Clone)]
pub struct Subject {
    /// The subject's unique ID.
    pub id: i16,
    /// The ID of the group the subject belongs to.
    pub group_id: i16,
    /// The subject's age.
    pub age: i8,
    /// The subject's length of stay.
    pub length_of_stay: i16,
}
