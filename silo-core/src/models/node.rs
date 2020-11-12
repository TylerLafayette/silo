#[derive(Debug)]
pub struct Node {
    has_parent: bool,
    parent_id: u64,
}

impl Node {
    pub fn new() -> Node {
        Node {}
    }

    pub fn new_with_parent(parent_id: u64) -> Node {
        Node {
            has_parent: true,
            parent_id: parent_id,
        }
    }
}
