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
