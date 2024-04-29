#[derive(Debug, PartialEq)]
pub(super) enum QuantityName {
    /// Безразмерная величина
    Dimensionless,
    /// Длина. Единица СИ - метр
    Length,
    /// Давление. Единица СИ - паскаль
    Pressure,
    /// Температура. Единица СИ - кельвин
    Temperature,
}
