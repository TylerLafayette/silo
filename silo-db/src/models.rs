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
