use super::super::{PhyQuantity, QuantityName};

const C_TO_K: f64 = 273.15;

impl PhyQuantity {
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
