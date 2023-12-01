use super::ID;

#[derive(Debug)]
pub struct IdCollection {
    #[allow(dead_code)]
    value: Vec<ID>,
}
impl IdCollection {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }
}
