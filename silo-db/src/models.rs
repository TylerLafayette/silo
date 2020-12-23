use oxidizer::*;

use silo_core::models;

/// Contains a single trait which can be applied to subjects.
#[derive(Entity, Default)]
pub struct SubjectTrait {
    /// The ID of the entity.
    #[primary_key]
    pub id: i16,

    /// The ID of the parent, if one exists.
    #[relation(model = "SubjectTrait", key = "id")]
    pub parent_id: i16,

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
    pub id: i16,

    #[relation(model = "Group", key = "id")]
    pub group_id: i16,

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
    pub id: i16,
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
    pub id: i16,

    #[relation(model = "Subject", key = "id")]
    pub subject_id: i16,

    #[relation(model = "SubjectTrait", key = "id")]
    pub subject_trait_id: i16,
}
