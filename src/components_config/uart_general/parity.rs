//! Добавлять бит четности

/// Добавлять бит четности
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub enum Parity {
    #[default]
    None,
    Even,
    Odd,
}
