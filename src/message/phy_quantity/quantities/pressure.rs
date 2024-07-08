use super::super::{PhyQuantity, QuantityName};

impl PhyQuantity {
    /// Задать давление в [Па]
    pub fn new_pressure_Pa(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Pressure,
        }
    }

    /// Получить давление в [Па]
    pub fn pressure_Pa(&self) -> f64 {
        match self.quantity_name {
            QuantityName::Pressure => self.value,
            _ => {
                let err = format!(
                    "You exprect pressure, but current unit: {:?}",
                    self.quantity_name
                );
                panic!("{err}")
            }
        }
    }
}
