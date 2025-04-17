use super::super::PhyQuantity;

impl std::ops::Mul<f64> for PhyQuantity {
    type Output = PhyQuantity;

    fn mul(self, rhs: f64) -> Self::Output {
        PhyQuantity {
            value: self.value * rhs,
            quantity_name: self.quantity_name,
        }
    }
}

impl std::ops::Mul<PhyQuantity> for f64 {
    type Output = PhyQuantity;

    fn mul(self, rhs: PhyQuantity) -> Self::Output {
        PhyQuantity {
            value: self * rhs.value,
            quantity_name: rhs.quantity_name,
        }
    }
}
