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
    pub fn temperature_C(&self) -> Result<f64, String> {
        match self.quantity_name {
            QuantityName::Temperature => {
                let value = self.value - C_TO_K;
                Ok(value)
            }
            _ => {
                let err = format!(
                    "You exprect temperature, but current unit: {:?}; value: {:?}",
                    self.quantity_name, self
                );
                Err(err.to_string())
            }
        }
    }
}
