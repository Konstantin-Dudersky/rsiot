//! Кол-во бит данных

/// Кол-во бит данных
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub enum DataBits {
    _5,
    _6,
    _7,
    #[default]
    _8,
}
