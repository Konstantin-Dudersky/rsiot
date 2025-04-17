use crate::message::phy_quantity::quantity_name::QuantityName;

use super::super::PhyQuantity;

impl std::ops::Div<PhyQuantity> for PhyQuantity {
    type Output = PhyQuantity;

    fn div(self, rhs: Self) -> Self::Output {
        let quantity_name = match self.quantity_name {
            QuantityName::Dimensionless => todo!(),
            QuantityName::Length => todo!(),
            QuantityName::Pressure => match rhs.quantity_name {
                QuantityName::Dimensionless => todo!(),
                QuantityName::Length => todo!(),
                QuantityName::Pressure => QuantityName::Dimensionless,
                QuantityName::Temperature => todo!(),
            },
            QuantityName::Temperature => todo!(),
        };

        PhyQuantity {
            value: self.value / rhs.value,
            quantity_name,
        }
    }
}
