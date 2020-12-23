/// Contains a single trait which can be applied to subjects.
pub struct SubjectTrait {
    /// The ID of the entity.
    pub id: i16,
    /// The ID of the parent, if one exists.
    pub parent_id: i16,
    /// The name of the trait.
    pub trait_name: String,
}
