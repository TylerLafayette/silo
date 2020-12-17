/// Contains information about a single subject for analysis.
#[derive(Debug)]
pub struct Subject {
    /// The subject's unique ID.
    pub id: i32,
    /// The ID of the group the subject belongs to.
    pub group_id: i32,
    /// The subject's age.
    pub age: i8,
    /// The subject's length of stay.
    pub length_of_stay: i16,
}
