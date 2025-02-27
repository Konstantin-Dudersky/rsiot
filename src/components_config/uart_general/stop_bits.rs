//! Кол-во стоповых бит

/// Кол-во стоповых бит
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub enum StopBits {
    #[default]
    _1,
    _1p5,
    _2,
}

impl From<StopBits> for f64 {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::_1 => 1.0,
            StopBits::_1p5 => 1.5,
            StopBits::_2 => 2.0,
        }
    }
}

#[cfg(feature = "cmp_esp")]
impl From<StopBits> for esp_idf_svc::hal::uart::config::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::_1 => Self::STOP1,
            StopBits::_1p5 => Self::STOP1P5,
            StopBits::_2 => Self::STOP2,
        }
    }
}
