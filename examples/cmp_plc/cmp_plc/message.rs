use rsiot::message::{Deserialize, MsgDataBound, Serialize, TimeToLive};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    OutputValue(u16),
}

impl TimeToLive for Data {}

impl MsgDataBound for Data {}
