use rsiot::message::{Deserialize, MsgDataBound, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    OutputValue(u16),
}

impl MsgDataBound for Data {}
