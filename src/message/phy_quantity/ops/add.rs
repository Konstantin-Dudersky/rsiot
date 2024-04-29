use super::super::PhyQuantity;

impl std::ops::Add for PhyQuantity {
    type Output = PhyQuantity;

    fn add(self, rhs: Self) -> Self::Output {
        if self.quantity_name != rhs.quantity_name {
            panic!(
                "Wrong operation: {:?} + {:?}",
                self.quantity_name, rhs.quantity_name
            )
        }
        Self {
            value: self.value + rhs.value,
            quantity_name: self.quantity_name,
        }
    }
}
