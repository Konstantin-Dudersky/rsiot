use super::super::{PhyQuantity, QuantityName};

impl PhyQuantity {
    /// Получить безразмерную величину
    pub fn dimensionless(&self) -> Result<f64, String> {
        match self.quantity_name {
            QuantityName::Dimensionless => {
                let value = self.value;
                Ok(value)
            }
            _ => {
                let err = format!(
                    "You exprect dimensionless, but current unit: {:?}",
                    self.quantity_name
                );
                Err(err.to_string())
            }
        }
    }
}
