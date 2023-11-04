#[derive(Debug)]
pub enum Error {
    Deserialization(String),
    Serialization(String),
}
