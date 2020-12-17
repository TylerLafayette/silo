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
    pub age: i8,
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
pub struct Group {
    #[primary_key]
    pub id: i32,
}
