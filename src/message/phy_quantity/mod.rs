//! Представление физической величины

//! TODO - В дальнейшем удалить. Использовать крейт rsiot-physical-quantities
#![deprecated]

mod ops;
mod quantities;
mod quantity_name;

use quantity_name::QuantityName;
use serde::{Deserialize, Serialize};

/// Физическая величина
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PhyQuantity {
    pub(crate) value: f64,
    /// Тип физической величины
    pub quantity_name: QuantityName,
}

impl PhyQuantity {
    /// Задать безразмероное значение
    pub fn new(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Dimensionless,
        }
    }
}
