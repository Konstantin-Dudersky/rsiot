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

#[cfg(feature = "cmp_esp")]
impl From<Parity> for esp_idf_svc::hal::uart::config::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => Self::ParityNone,
            Parity::Even => Self::ParityEven,
            Parity::Odd => Self::ParityOdd,
        }
    }
}
