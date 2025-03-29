/// Формат сериализации / десериализации
#[derive(Clone, Copy, Debug)]
pub enum SerdeAlgKind {
    /// JSON
    Json,
    /// TOML
    Toml,
}
