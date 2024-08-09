use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum QuantityName {
    /// Безразмерная величина
    #[default]
    Dimensionless,
    /// Длина. Единица СИ - метр
    Length,
    /// Давление. Единица СИ - паскаль
    Pressure,
    /// Температура. Единица СИ - кельвин
    Temperature,
}
