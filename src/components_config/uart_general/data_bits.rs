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

impl From<DataBits> for f64 {
    fn from(value: DataBits) -> Self {
        match value {
            DataBits::_5 => 5.0,
            DataBits::_6 => 6.0,
            DataBits::_7 => 7.0,
            DataBits::_8 => 8.0,
        }
    }
}
