use crate::id::ID;

#[derive(Debug)]
pub struct Node {
    #[allow(dead_code)]
    id: ID,
    #[allow(dead_code)]
    label: String,
}
impl Node {
    pub fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}
