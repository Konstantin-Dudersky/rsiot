#![allow(non_snake_case)]

use super::QuantityName;

const C_TO_K: f64 = 273.15;

/// Физическая величина
#[derive(Debug)]
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

    // Length --------------------------------------------------------------------------------------

    /// Задать длину в [м]
    pub fn new_length_M(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Length,
        }
    }

    // Pressure ------------------------------------------------------------------------------------

    /// Задать давление в [Па]
    pub fn new_pressure_Pa(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Pressure,
        }
    }

    // Temperature ---------------------------------------------------------------------------------

    /// Задать температуру в [℃]
    pub fn new_temperature_C(value: f64) -> Self {
        Self {
            value: value + C_TO_K,
            quantity_name: QuantityName::Temperature,
        }
    }

    /// Получить температуру в [℃]
    pub fn temperature_C(&self) -> f64 {
        match self.quantity_name {
            QuantityName::Temperature => self.value - C_TO_K,
            _ => {
                let err = format!(
                    "You exprect temperature, but current unit: {:?}",
                    self.quantity_name
                );
                panic!("{err}")
            }
        }
    }
}
