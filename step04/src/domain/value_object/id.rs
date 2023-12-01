#[derive(Debug)]
pub struct ID {
    #[allow(dead_code)]
    value: String,
}
impl ID {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
