use super::super::{PhyQuantity, QuantityName};

impl PhyQuantity {
    /// Задать давление в \[Па\]
    pub fn new_pressure_Pa(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Pressure,
        }
    }

    /// Получить давление в \[Па\]
    pub fn pressure_Pa(&self) -> Result<f64, String> {
        match self.quantity_name {
            QuantityName::Pressure => {
                let value = self.value;
                Ok(value)
            }
            _ => {
                let err = format!(
                    "You exprect pressure, but current unit: {:?}",
                    self.quantity_name
                );
                Err(err.to_string())
            }
        }
    }

    /// Получить давление в \[мм рт столба\]
    pub fn pressure_mmHg(&self) -> Result<f64, String> {
        match self.quantity_name {
            QuantityName::Pressure => {
                let value = self.value * 0.00750063755419211;
                Ok(value)
            }
            _ => {
                let err = format!(
                    "You exprect pressure, but current unit: {:?}",
                    self.quantity_name
                );
                Err(err.to_string())
            }
        }
    }
}
