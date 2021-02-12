use serde::Serialize;

/// Contains a single trait which can be applied to subjects.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectTrait {
    /// The ID of the entity.
    pub id: i32,
    /// The ID of the parent, if one exists.
    pub parent_id: i32,
    /// The name of the trait.
    pub trait_name: String,
}
