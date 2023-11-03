#[derive(Debug)]
pub enum Errors {
    Deserialization(String),
    Serialization(String),
}
