use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum QuantityName {
    /// Безразмерная величина
    Dimensionless,
    /// Длина. Единица СИ - метр
    Length,
    /// Давление. Единица СИ - паскаль
    Pressure,
    /// Температура. Единица СИ - кельвин
    Temperature,
}
