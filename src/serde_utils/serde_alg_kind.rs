// ANCHOR: SerdeAlgKind
/// Формат сериализации / десериализации
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SerdeAlgKind {
    /// CBOR
    #[cfg(feature = "serde_cbor")]
    Cbor,

    /// JSON
    #[cfg(feature = "serde_json")]
    Json,

    /// MessagePack
    #[cfg(feature = "serde_messagepack")]
    MessagePack,

    /// Postcard
    #[cfg(feature = "serde_postcard")]
    Postcard,

    /// TOML
    #[cfg(feature = "serde_toml")]
    Toml,

    /// Алгоритм не задан
    #[default]
    Unspecified,
}
// ANCHOR: SerdeAlgKind
