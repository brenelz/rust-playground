use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Product {
    title: String,
}
impl Product {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
