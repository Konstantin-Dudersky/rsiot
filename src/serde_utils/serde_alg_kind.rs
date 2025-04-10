/// Формат сериализации / десериализации
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SerdeAlgKind {
    /// JSON
    #[default]
    Json,
    /// Postcard
    Postcard,
    /// TOML
    Toml,
}
