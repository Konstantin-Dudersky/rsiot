use super::super::{PhyQuantity, QuantityName};

impl PhyQuantity {
    /// Задать длину в \[м\]
    pub fn new_length_m(value: f64) -> Self {
        Self {
            value,
            quantity_name: QuantityName::Length,
        }
    }
}
