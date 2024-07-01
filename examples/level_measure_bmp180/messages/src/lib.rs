use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, PhyQuantity};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Pressure1(PhyQuantity),
    Pressure2(PhyQuantity),
    WaterLevel2(f64),
}

impl MsgDataBound for Custom {}
