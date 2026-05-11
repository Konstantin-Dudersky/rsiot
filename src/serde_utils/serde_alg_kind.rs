// ANCHOR: SerdeAlgKind
/// Формат сериализации / десериализации
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SerdeAlgKind {
    /// CBOR
    ///
    /// Необходимо активировать feature `serde_cbor`
    #[cfg(feature = "serde_cbor")]
    Cbor,

    /// JSON
    ///
    /// Необходимо активировать feature `serde_json`
    #[cfg(feature = "serde_json")]
    Json,

    /// MessagePack
    ///
    /// Необходимо активировать feature `serde_messagepack`
    #[cfg(feature = "serde_messagepack")]
    MessagePack,

    /// Postcard
    ///
    /// Необходимо активировать feature `serde_postcard`
    #[cfg(feature = "serde_postcard")]
    Postcard,

    /// TOML
    ///
    /// Необходимо активировать feature `serde_toml`
    #[cfg(feature = "serde_toml")]
    Toml,

    /// Алгоритм не задан
    #[default]
    Unspecified,
}
// ANCHOR: SerdeAlgKind
