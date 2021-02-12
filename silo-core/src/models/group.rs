use serde::Serialize;

/// Groups multiple subjects together.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    /// The group's unique ID.
    pub id: i32,
}
