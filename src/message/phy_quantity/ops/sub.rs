use tracing::warn;

use super::super::PhyQuantity;

impl std::ops::Sub for PhyQuantity {
    type Output = PhyQuantity;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.quantity_name != rhs.quantity_name {
            warn!(
                "Wrong operation: {:?} - {:?}",
                self.quantity_name, rhs.quantity_name
            )
        }
        Self {
            value: self.value - rhs.value,
            quantity_name: self.quantity_name,
        }
    }
}
