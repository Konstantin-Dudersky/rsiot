use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Data {
    InjectPeriodic(bool),
    OutputValue(u16),
}

impl MsgDataBound for Data {}
